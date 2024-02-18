use maud::{html, Markup, Render};

use crate::{
    model::{news::NewsItem, site_data::SiteData},
    templates::*,
};

pub struct Homepage<'a> {
    sd: &'a SiteData,
}

impl<'a> Homepage<'a> {
    pub fn new(sd: &'a SiteData) -> Self {
        Self { sd }
    }
}

impl Render for Homepage<'_> {
    fn render(&self) -> Markup {
        let content = html! {
            main .homepage .container {
                h1 { "welcome to the site" }
                p { "please enjoy the site" }
                (news_box(&self.sd.news))
            }
        };
        let base = Base {
            title: "Homepage".into(),
            navbar: Navbar { highlighted: None },
            content,
        };

        base.render()
    }
}

fn news_box<'a>(items: impl IntoIterator<Item = &'a NewsItem>) -> Markup {
    fn news_item(item: &NewsItem) -> Markup {
        html! {
            div .item {
                @if let Some(h) = &item.title {
                    h3 .title { (h) }
                }
                span .date { (util::format_dt(item.time)) }
                (item.content)
            }
        }
    }

    html! {
        div .newsbox {
            header {
                h2 .title { "News" }
            }
            div .items {
                @for item in items {
                    (news_item(item))
                }
            }
        }
    }
}
