use maud::{html, Markup, Render};

use crate::{
    model::{button88x31::Button88x31, news::NewsItem, site_data::SiteData},
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
                (buttons(&self.sd.buttons))
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

fn buttons<'a>(buttons: impl IntoIterator<Item = &'a Button88x31>) -> Markup {
    fn button(b: &Button88x31) -> Markup {
        let img = html! {
            img src=(b.img) alt=[&b.alt] title=[&b.title];
        };
        match &b.href {
            Some(href) => html! {
                a href=(href) { (img) }
            },
            None => img,
        }
    }

    html! {
        div .buttons {
            @for b in buttons {
                (button(b))
            }
        }
    }
}
