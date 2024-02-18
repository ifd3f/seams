use comrak::nodes::{AstNode, NodeHtmlBlock, NodeValue};
use itertools::Itertools;

use crate::{errors::Errors, transform::katex};

use super::katex::{KatexError, MathMode};

pub async fn apply_katex<'a>(node: &'a AstNode<'a>) -> Result<(), Errors<KatexError>> {
    let mut errors = Errors::new();
    apply_block_katex(node)
        .await
        .unwrap_or_else(|es| errors.extend(es));
    apply_inline_katex(node)
        .await
        .unwrap_or_else(|es| errors.extend(es));
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn apply_inline_katex<'a>(node: &'a AstNode<'a>) -> Result<(), Errors<KatexError>> {
    let mut errors = Errors::new();

    // Search for all the opening tags
    let mut to_visit = node.children().collect_vec();
    let mut opentags = vec![];
    while let Some(n) = to_visit.pop() {
        let borrowed = n.data.borrow();

        match &borrowed.value {
            NodeValue::HtmlInline(s) if s == MathMode::Inline.opening_tag() => {
                opentags.push((n, MathMode::Inline));
            }
            NodeValue::HtmlInline(s) if s == MathMode::Display.opening_tag() => {
                opentags.push((n, MathMode::Display));
            }
            _ => to_visit.extend(n.children()),
        }
    }

    // From each opening tag, search for closing tags
    for (open_tag, mode) in opentags {
        let Some((between, closing)) = find_closing_tag(open_tag, mode) else {
            continue;
        };

        // Build up the katex source from the nodes between
        let mut katex = String::new();
        for n in &between {
            katex.push_str(match &n.data.borrow().value {
                NodeValue::Text(s) => s.as_str(),
                NodeValue::SoftBreak => "\n",
                _ => " ",
            })
        }

        // Transform that katex source
        let html = match katex::transform_math(&katex, mode).await {
            Ok(r) => r,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        // Replace the first tag with inline html
        open_tag.data.borrow_mut().value = NodeValue::HtmlInline(html);

        // Detach the other nodes
        for n in between {
            n.detach();
        }
        closing.detach();
    }

    Ok(())
}

/// Search siblings of the open tag until it reaches its closing tag. Returns
/// all nodes strictly between the closing tag in a vec, plus the closing tag itself.
fn find_closing_tag<'a>(
    open_tag: &'a AstNode<'a>,
    mode: MathMode,
) -> Option<(Vec<&'a AstNode<'a>>, &'a AstNode<'a>)> {
    let mut visited_nodes = vec![];
    let mut sibling_iter = open_tag.next_sibling();

    while let Some(sib) = sibling_iter {
        match &sib.data.borrow().value {
            NodeValue::HtmlInline(t) if t == mode.closing_tag() => {
                return Some((visited_nodes, sib))
            }
            _ => (),
        }

        sibling_iter = sib.next_sibling();
        visited_nodes.push(sib);
    }
    None
}

#[tracing::instrument(skip_all)]
async fn apply_block_katex<'a>(node: &'a AstNode<'a>) -> Result<(), Errors<KatexError>> {
    let mut errors = Errors::new();
    let mut to_visit = node.children().collect_vec();
    while let Some(n) = to_visit.pop() {
        let mut borrowed = n.data.borrow_mut();

        match &mut borrowed.value {
            NodeValue::HtmlBlock(b) => {
                let Some((math, mode)) = get_math_html(&b.literal) else {
                    continue;
                };
                match katex::transform_math(math, mode).await {
                    Ok(html) => b.literal = html_math_block(&html),
                    Err(e) => errors.push(e),
                }
            }
            NodeValue::CodeBlock(cb) if cb.info == "math" => {
                match katex::transform_math(&cb.literal, MathMode::Display).await {
                    Ok(html) => {
                        borrowed.value = NodeValue::HtmlBlock(NodeHtmlBlock {
                            block_type: 0,
                            literal: html_math_block(&html),
                        });
                    }
                    Err(e) => errors.push(e),
                }
            }
            _ => to_visit.extend(n.children()),
        }
    }

    Ok(())
}

fn get_math_html(html: &str) -> Option<(&str, MathMode)> {
    let display = || {
        Some((
            html.strip_prefix("<M>")?.strip_suffix("</M>")?,
            MathMode::Display,
        ))
    };
    let inline = || {
        Some((
            html.strip_prefix("<m>")?.strip_suffix("</m>")?,
            MathMode::Inline,
        ))
    };
    display().or_else(inline)
}

fn html_math_block(html: &str) -> String {
    maud::html! {
        div .math-block { (html) }
    }
    .into_string()
}

#[cfg(test)]
mod tests {
    use comrak::{format_html_with_plugins, parse_document, Arena};

    use crate::transform::markdown::make_md_options;

    use super::*;

    const MULTIPLE_KATEX_STR: &str =
        "Let the third-order Taylor series approximation of <m>T_1(t)</m> be the cubic
function <m>g(t)=at^3+bt^2+ct+d</m>. The blue function on the graph is the Taylor
series approximation of it. I decided that the simulator would have 16
increments, so the actual function used is <m>g(t)</m> rescaled to <m>[0,16]</m> centered
at 8:";

    #[tokio::test]
    pub async fn katex_transforms_correctly() {
        let mut arena = Arena::new();
        let md = MULTIPLE_KATEX_STR;
        let options = make_md_options();
        let root = parse_document(&mut arena, md, &options);

        eprintln!("BEFORE TRANFORM: {root:#?}");
        apply_katex(root).await.unwrap();
        eprintln!("AFTER TRANFORM: {root:#?}");

        let mut html = vec![];
        format_html_with_plugins(root, &options, &mut html, &comrak::Plugins::default()).unwrap();
        let html = String::from_utf8_lossy(&html);

        // root.descendants().contains(|n| match n)
        assert!(
            !html.contains("<m>"),
            "We did not transform correctly.\nFull html: {}",
            html
        );
    }
}
