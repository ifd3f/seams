use maud::{html, Markup, PreEscaped};

use crate::{
    load::document::FullyLoadedDocument,
    model::{metadata::ArbitraryPage, SiteData, SiteIndex},
};

use super::{BaseTemplatePage, PageMeta};

#[derive(derive_more::From)]
pub struct ArbitraryPageRender<'a> {
    page: &'a FullyLoadedDocument<ArbitraryPage>,
}

impl<'a> BaseTemplatePage for ArbitraryPageRender<'a> {
    fn render_page(&self, _sd: &SiteData, _si: &SiteIndex<'_>) -> (PageMeta, Markup) {
        let meta = PageMeta {
            title: self.page.meta().title.clone(),
            navbar_path: self.page.meta().navbar_path.clone(),
            href: self.page.meta().slug.clone(),
            extra_head: html! {
                meta property="og:title" content=(self.page.meta().title);
                @if let Some(d) = &self.page.meta().meta_description {
                    meta property="og:description" content=(d);
                }
                meta property="og:type" content="article";
                meta property="og:url" content=(format!("https://astrid.tech{}", self.page.meta().slug));
                @for t in &self.page.meta().tags {
                    meta property="article:tag" content=(t);
                }
            },
            ..Default::default()
        };
        let content = html! {
            main .container-md {
                (PreEscaped(self.page.html()))
            }
        };
        (meta, content)
    }
}
