use std::{borrow::{Borrow, BorrowMut}, fmt::Display};

use scraper::{ElementRef, Html, Node};
use typed_arena::Arena;

use crate::errors::Errors;

use super::katex::{find_katex, transform_katex_str, KatexError};

pub fn process_html(html: &str) -> anyhow::Result<String> {
    let  dom = Html::parse_fragment(html);    
    todo!();
}

#[tracing::instrument(skip_all)]
pub async fn postprocess_katex<'a>(root: ElementRef<'a>) -> Result<(), Errors<KatexError>> {
    let mut errors = Errors::new();

    let mut to_visit = root.children().into_iter().collect::<Vec<_>>();
    while let Some(n) = to_visit.pop() {
        if let Some(t) = n.value().as_text() {

        }
        match n {
            Node::Text(t) => {
                let transformed = match transform_katex_str(t).await {
                    Ok(s) => s,
                    Err(es) => {
                        errors.extend(es.0.into());
                        continue;
                    }
                };

                /*
                let tree = Dom::parse(&transformed)?;
                *n = Node::Element(Element {
                    name: "div".into(),
                    classes: vec!["transformed-math".into()],
                    children: tree.children,
                    ..Default::default()
                }); */
            }
            Node::Element(e) => {
                to_visit.extend(e.children.iter_mut());
            }
            Node::Comment(_) => (),
        }
    }

    if errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub struct HtmlErrors(pub Vec<HtmlErrorKind>);

impl Display for HtmlErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum HtmlErrorKind {
    #[error("katex error: {0}")]
    KatexError(#[from] KatexError),
}
