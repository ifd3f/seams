use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    io::BufWriter,
};

use comrak::{
    arena_tree::Node,
    format_html_with_plugins,
    nodes::{Ast, AstNode, LineColumn, NodeLink, NodeValue, Sourcepos},
    parse_document,
    plugins::syntect::SyntectAdapter,
    Arena, PluginsBuilder, RenderPluginsBuilder,
};
use futures::{stream::FuturesOrdered, StreamExt, TryFutureExt};

use htmlentity::entity::EncodeType;
use itertools::Itertools;
use vfs::VfsError;

use crate::{
    errors::Errors,
    media::{Media, MediaRegistry},
};

use super::{
    common::TransformContext,
    graphviz::{transform_graphviz, GraphvizError},
    katex::{find_katex, transform_katex_str, transform_math, Block, KatexError},
};

pub fn make_md_options() -> comrak::Options {
    let mut md_options = comrak::Options::default();
    md_options.extension.strikethrough = true;
    md_options.extension.table = true;
    md_options.extension.autolink = true;
    md_options.extension.tasklist = true;
    md_options.extension.superscript = true;
    md_options.extension.footnotes = true;
    md_options.extension.header_ids = Some("header-".into());
    md_options.parse.smart = true;
    md_options.render.github_pre_lang = true;
    md_options.render.width = 80;
    md_options.render.unsafe_ = true;
    md_options.render.escape = false;
    md_options
}

/// Transform markdown into HTML. May be computationally expensive.
#[tracing::instrument(skip_all)]
pub async fn transform_markdown<'a>(
    ctx: &'a TransformContext<'a>,
    raw: &'a str,
) -> Result<String, Errors<MarkdownError>> {
    let mut arena = Arena::new();

    let md_options = make_md_options();

    let syntect = SyntectAdapter::new(Some("base16-ocean.dark"));
    let plugins = PluginsBuilder::default()
        .render(
            RenderPluginsBuilder::default()
                .codefence_syntax_highlighter(Some(&syntect))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let root = parse_document(&mut arena, raw, &md_options);

    let mut errors = Errors::new();

    if let Err(es) = apply_graphviz(&ctx.media(), root).await {
        errors.extend(es)
    }

    if let Err(es) = relink_images(ctx, root) {
        errors.extend(es)
    }

    /*
    let images = arena
        .iter_mut()
        .filter_map(|n| match n.data.borrow().value {
            NodeValue::Image(link) => Some(content_root.join(link.url)),
            _ => None,
        });

    let graphviz = arena
        .iter_mut()
        .filter_map(|n| match n.data.borrow().value {
            NodeValue::CodeBlock(cb) => {
                if cb.info == "dot" {
                    Some(make_graphviz(literal))
                }
                None
            },
            _ => None,
        });
        */
    let mut bw = BufWriter::new(Vec::new());
    format_html_with_plugins(root, &md_options, &mut bw, &plugins).unwrap();
    apply_katex(&mut arena, root);

    let raw = &bw.into_inner().unwrap();
    let entity_escaped = htmlentity::entity::encode(
        &raw,
        &EncodeType::NamedOrHex,
        &htmlentity::entity::CharacterSet::NonASCII,
    );
    Ok(String::from_utf8(entity_escaped.into_bytes()).unwrap())
}

/// Transform links in images into what they should be, and upload them.
#[tracing::instrument(skip_all)]
pub fn relink_images<'a>(
    ctx: &'a TransformContext,
    root: &'a AstNode<'a>,
) -> Result<(), Errors<MarkdownError>> {
    let mut errors = Errors::new();

    for n in root.descendants() {
        let mut ast = n.data.borrow_mut();
        match &mut ast.value {
            NodeValue::Image(link) if link.url.starts_with("./") => {
                let mut f = move || {
                    let image = ctx.content_root().join(&link.url)?;
                    link.url = ctx
                        .media()
                        .upload_media(image)
                        .map_err(MarkdownErrorKind::Image)?;
                    Ok(())
                };
                if let Err(e) = f() {
                    errors.push(MarkdownError::new(ast.sourcepos, e))
                }
            }
            _ => (),
        };
    }

    errors.into_result()?;

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn apply_graphviz<'a>(
    media: &'a MediaRegistry,
    root: &'a AstNode<'a>,
) -> Result<(), Errors<MarkdownError>> {
    let mut errors = Errors::new();

    for n in root.descendants() {
        let cell = &n.data;

        let (position, literal, info) = {
            let ast = cell.borrow();
            let sourcepos = ast.sourcepos.clone();
            let (info, literal) = match &ast.value {
                NodeValue::CodeBlock(cb) => match parse_graphviz_info(&cb.info) {
                    Some(info) => (info, cb.literal.clone()),
                    _ => continue,
                },
                _ => continue,
            };
            (sourcepos, literal, info)
        };

        let result = async move {
            let result = transform_graphviz(&literal).await?;
            let link = media
                .upload_media(Media {
                    filename: Some("graphviz.svg".into()),
                    mimetype: Some("image/svg+xml".parse().unwrap()),
                    body: result.into_bytes(),
                })
                .unwrap();
            let ast = NodeValue::Image(NodeLink {
                url: link,
                title: match info {
                    GraphvizInfo::Untitled => "Graphviz image".into(),
                    GraphvizInfo::Titled(title) => title,
                },
            });
            cell.borrow_mut().value = ast;
            Ok::<(), GraphvizError>(())
        }
        .map_err(move |e| MarkdownError::new(position.clone(), e.into()))
        .await;

        if let Err(e) = result {
            errors.push(e);
        }
    }

    errors.into_result()?;

    Ok(())
}

enum GraphvizInfo {
    Untitled,
    Titled(String),
}

fn parse_graphviz_info(infostr: &str) -> Option<GraphvizInfo> {
    if let Some(title) = infostr.strip_prefix("dot:") {
        Some(GraphvizInfo::Titled(title.into()))
    } else if infostr == "dot" {
        Some(GraphvizInfo::Untitled)
    } else {
        None
    }
}

#[tracing::instrument(skip_all)]
pub async fn apply_katex<'a>(
    arena: &'a mut Arena<AstNode<'a>>,
    node: &'a AstNode<'a>,
) -> Result<(), Errors<KatexError>> {
    let mut to_visit = node.children().collect_vec();

    while let Some(n) = to_visit.pop() {
        for c in n.children() {
            let nv = c.data.borrow().value.clone();
            let bmut = c.data.borrow_mut();
            match nv {
                NodeValue::Text(t) => {
                    for new in transform_text_katex_nodes(&t).await? {
                        let node = AstNode::new(RefCell::new(Ast::new(new, bmut.sourcepos.start)));
                        let arenaval = arena.alloc(node);
                        c.insert_before(arenaval);
                    }
                    c.detach();
                }
                _ => to_visit.push(c),
            }
        }
    }

    Ok(())
}

async fn transform_text_katex_nodes<'a>(
    text: &'a str,
) -> Result<Vec<NodeValue>, Errors<KatexError>> {
    let fu = find_katex(&text)
        .into_iter()
        .map(|b| async {
            Ok(match b {
                Block::Plain(t) => NodeValue::Text(t.to_owned()),
                Block::BlockMath(m) => NodeValue::HtmlInline(transform_math(&m, true).await?),
                Block::InlineMath(m) => NodeValue::HtmlInline(transform_math(&m, false).await?),
            })
        })
        .collect::<FuturesOrdered<_>>();

    let mut result = vec![];
    let mut errors = Errors::new();
    while let Some(n) = fu.next().await {
        match n {
            Ok(n) => result.push(n),
            Err(e) => errors.push(e),
        }
    }
    errors.into_result()?;
    Ok(result)
}

#[derive(thiserror::Error, Debug)]
pub struct MarkdownError {
    pos: Sourcepos,
    kind: MarkdownErrorKind,
}

impl MarkdownError {
    pub fn new(pos: Sourcepos, kind: MarkdownErrorKind) -> Self {
        Self { pos, kind }
    }
}

impl std::fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error at {}: {}", self.pos, self.kind)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MarkdownErrorKind {
    #[error("graphviz error: {0}")]
    Graphviz(#[from] GraphvizError),

    #[error("image error: {0}")]
    Image(anyhow::Error),

    #[error("fs error: {0}")]
    Vfs(#[from] VfsError),
}
