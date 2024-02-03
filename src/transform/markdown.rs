use std::cell::RefCell;

use comrak::{
    nodes::{Ast, AstNode, NodeHtmlBlock, NodeValue, Sourcepos},
    parse_document, Arena,
};
use futures::{stream::FuturesUnordered, StreamExt, TryFutureExt};
use vfs::VfsError;

use crate::loading::TransformedContent;

use super::{
    common::TransformContext,
    graphviz::{transform_graphviz, GraphvizError},
};

/// Transform markdown into HTML. May be computationally expensive.
pub async fn transform_markdown(
    ctx: &TransformContext,
    raw: String,
) -> Result<TransformedContent, Vec<MarkdownError>> {
    let mut arena = Arena::new();
    let md_options = comrak::Options::default();

    let root = parse_document(&mut arena, &raw, &md_options);

    let mut errors = vec![];

    if let Err(es) = apply_graphviz(root).await {
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
    todo!()
}

/// Transform links in images into what they should be, and upload them.
pub fn relink_images<'a>(
    ctx: &'a TransformContext,
    root: &'a AstNode<'a>,
) -> Result<(), Vec<MarkdownError>> {
    let mut errors = vec![];

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

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

pub async fn apply_graphviz<'a>(root: &'a AstNode<'a>) -> Result<(), Vec<MarkdownError>> {
    let mut jobs = FuturesUnordered::new();
    for n in root.descendants() {
        let cell = &n.data;

        let (position, literal) = {
            let ast = cell.borrow();
            let sourcepos = ast.sourcepos.clone();
            let literal = match &ast.value {
                NodeValue::CodeBlock(cb) if cb.info == "dot" => cb.literal.clone(),
                _ => continue,
            };
            (sourcepos, literal)
        };

        jobs.push(
            async move {
                let result = transform_graphviz(&literal).await?;
                let ast = NodeValue::HtmlBlock(NodeHtmlBlock {
                    block_type: 0,
                    literal: result,
                });
                cell.borrow_mut().value = ast;
                Ok::<(), GraphvizError>(())
            }
            .map_err(move |e| MarkdownError::new(position.clone(), e.into())),
        );
    }

    let mut errors = vec![];
    while let Some(r) = jobs.next().await {
        if let Err(e) = r {
            errors.push(e)
        }
    }

    if errors.is_empty() {
        return Err(errors);
    }

    Ok(())
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
