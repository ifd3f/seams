use std::{io::BufWriter};

use comrak::{
    format_html,
    nodes::{AstNode, NodeLink, NodeValue, Sourcepos},
    parse_document, Arena,
};
use futures::{stream::FuturesUnordered, StreamExt, TryFutureExt};

use vfs::VfsError;

use crate::media::{Media, MediaRegistry};

use super::{
    common::TransformContext,
    graphviz::{transform_graphviz, GraphvizError},
};

/// Transform markdown into HTML. May be computationally expensive.
pub async fn transform_markdown<'a>(
    ctx: &'a TransformContext<'a>,
    raw: &'a str,
) -> Result<String, MarkdownErrors> {
    let mut arena = Arena::new();
    let mut md_options = comrak::Options::default();
    md_options.render.unsafe_ = true;

    let root = parse_document(&mut arena, raw, &md_options);

    let mut errors = MarkdownErrors::default();

    if let Err(es) = apply_graphviz(&ctx.media(), root).await {
        errors.0.extend(es.0)
    }

    if let Err(es) = relink_images(ctx, root) {
        errors.0.extend(es.0)
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
    format_html(root, &md_options, &mut bw).unwrap();
    Ok(String::from_utf8(bw.into_inner().unwrap()).unwrap())
}

/// Transform links in images into what they should be, and upload them.
pub fn relink_images<'a>(
    ctx: &'a TransformContext,
    root: &'a AstNode<'a>,
) -> Result<(), MarkdownErrors> {
    let mut errors = MarkdownErrors::default();

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
                    errors.0.push(MarkdownError::new(ast.sourcepos, e))
                }
            }
            _ => (),
        };
    }

    if !errors.0.is_empty() {
        return Err(errors);
    }

    Ok(())
}

pub async fn apply_graphviz<'a>(
    media: &'a MediaRegistry,
    root: &'a AstNode<'a>,
) -> Result<(), MarkdownErrors> {
    let mut jobs = FuturesUnordered::new();
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

        jobs.push(
            async move {
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
            .map_err(move |e| MarkdownError::new(position.clone(), e.into())),
        );
    }

    let mut errors = MarkdownErrors::default();
    while let Some(r) = jobs.next().await {
        if let Err(e) = r {
            errors.0.push(e)
        }
    }

    if errors.0.is_empty() {
        return Err(errors);
    }

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

#[derive(Default, Debug)]
pub struct MarkdownErrors(pub Vec<MarkdownError>);

impl std::error::Error for MarkdownErrors {}

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

impl std::fmt::Display for MarkdownErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.0 {
            writeln!(f, "{e}")?;
        }
        Ok(())
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
