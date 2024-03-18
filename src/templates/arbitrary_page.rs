use maud::{html, Markup};

use crate::{
    load::document::FullyLoadedDocument,
    model::{metadata::ArbitraryPage, SiteData, SiteIndex},
};

use super::{BaseTemplatePage, NavbarItem, PageMeta};

pub struct ArbitraryPageRender<'a> {
    page: &'a FullyLoadedDocument<ArbitraryPage>,
}

impl<'a> BaseTemplatePage for ArbitraryPageRender<'a> {
    fn render_page(&self, sd: &SiteData, si: &SiteIndex<'_>) -> (PageMeta, Markup) {
        let meta = PageMeta {
            title: self.page.meta().title.clone(),
            navbar_highlighted: NavbarItem::Blog.into(),
            extra_head: html! {
                meta property="og:title" content=(self.post.meta().title);
                @if let Some(t) = &self.post.meta().tagline {
                    meta property="og:description" content=(t);
                }
                meta property="og:type" content="article";
                meta property="og:url" content=(format!("https://astrid.tech{}", self.post.meta().href()));
                meta property="article:published_time" content=(self.post.meta().date.published.to_rfc3339());
                @for t in &self.post.meta().tags {
                    meta property="article:tag" content=(t);
                }
            },
            ..Default::default()
        };
        let content = html! {
            main .container .longform {
                (self.page_content(&sd.tags))
            }
        };
        (meta, content)
    }
}
