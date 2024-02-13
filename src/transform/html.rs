/*
pub fn process_html(html: &str) -> anyhow::Result<String> {
    let mut dom = Dom::parse(html)?;

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn postprocess_katex<'a>(dom: &mut Dom) -> Result<(), anyhow::Error> {
    let mut errors = vec![];

    let mut to_visit = dom.children.iter_mut().collect::<Vec<_>>();
    while let Some(n) = to_visit.pop() {
        match n {
            html_parser::Node::Text(_) => todo!(),
            html_parser::Node::Element(_) => todo!(),
            html_parser::Node::Comment(_) => todo!(),
        }
    }

    if errors.0.is_empty() {
        return Err(errors);
    }

    Ok(())
}
 */
