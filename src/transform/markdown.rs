use comrak::{
    nodes::{AstNode, NodeCodeBlock, NodeHtmlBlock, NodeValue},
    parse_document, Arena,
};
use vfs::{VfsError, VfsPath};

use crate::{loading::TransformedContent, media::MediaRegistry};

use super::graphviz::{transform_graphviz, GraphvizError};

/// Transform markdown into HTML. May be computationally expensive.
pub async fn transform_markdown(
    content_root: VfsPath,
    media: Box<dyn MediaRegistry>,
    raw: String,
) -> anyhow::Result<TransformedContent> {
    let mut arena = Arena::new();
    let md_options = comrak::Options::default();

    let mut root = parse_document(&mut arena, &raw, &md_options);

    apply_graphviz(&mut root).await?;
    apply_images(content_root, media, &mut root).await?;

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

/// Transform links in images into what they should be.
pub async fn apply_images(
    content_root: VfsPath,
    media: &dyn MediaRegistry,
    node: &mut AstNode<'_>,
) -> anyhow::Result<()> {
    if let NodeValue::Image(link) = &mut node.data.borrow_mut().value {
        if link.url.starts_with("./") {
            let image = content_root.join(&link.url)?;
            link.url = media.upload(image).await?;
        }
    } else {
        for n in node.children() {
            apply_images(content_root, media, node).await?;
        }
    }

    Ok(())
}

pub async fn apply_graphviz<'a>(node: &'a mut AstNode<'a>) -> Result<(), GraphvizError> {
    let literal = match &node.value {
        NodeValue::CodeBlock(NodeCodeBlock { info, literal, .. }) if info == "dot" => literal,
        _ => {
            for n in &mut node.child {
                apply_graphviz(n);
                return Ok(());
            }
        }
    };

    let svg = transform_graphviz(&literal).await?;
    node.value = NodeValue::HtmlBlock(NodeHtmlBlock {
        block_type: 0,
        literal: svg,
    });

    Ok(())
}
