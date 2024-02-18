use comrak::nodes::{AstNode, NodeValue};
use itertools::Itertools;

use crate::{errors::Errors, transform::katex};

use super::katex::KatexError;

#[tracing::instrument(skip_all)]
pub async fn apply_katex<'a>(node: &'a AstNode<'a>) -> Result<(), Errors<KatexError>> {
    fn get_math_html(html: &str) -> Option<(&str, bool)> {
        let display = || Some((html.strip_prefix("<M>")?.strip_suffix("</M>")?, true));
        let non_display = || Some((html.strip_prefix("<m>")?.strip_suffix("</m>")?, false));
        display().or_else(non_display)
    }

    let mut errors = Errors::new();
    let mut to_visit = node.children().collect_vec();
    while let Some(n) = to_visit.pop() {
        let mut borrowed = n.data.borrow_mut();

        match &mut borrowed.value {
            NodeValue::HtmlBlock(b) => {
                if let Some((math, mode)) = get_math_html(&b.literal) {
                    match katex::transform_math(math, mode).await {
                        Ok(html) => {
                            b.literal = maud::html! {
                                div .math-block { (html) }
                            }
                            .into_string()
                        }
                        Err(e) => errors.push(e),
                    }
                }
            }
            NodeValue::HtmlInline(h) => {
                if let Some((math, mode)) = get_math_html(h) {
                    match katex::transform_math(math, mode).await {
                        Ok(html) => {
                            *h = html;
                        }
                        Err(e) => errors.push(e),
                    }
                }
            }
            /*
            NodeValue::CodeBlock(cb) if cb.info == "math" => {
                if let Some((mode, math)) = get_math_html(&cb.literal) {
                    match katex::transform_math(math, mode).await {
                        Ok(m) => {
                            let html = maud::html! {
                                div .math-block {
                                    (m)
                                }
                            }
                            .into_string();
                        }
                        Err(e) => errors.push(e),
                    }
                }
            }
            */
            _ => to_visit.extend(n.children()),
        }
    }

    Ok(())
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
