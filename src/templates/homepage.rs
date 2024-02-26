use itertools::Itertools;
use maud::{html, Markup};

use crate::{
    model::{
        button88x31::Button88x31,
        news::NewsItem,
        site_data::{SiteData, SiteIndex},
    },
    templates::*,
};

pub const MAX_POSTS_ON_FRONT_PAGE: usize = 5;

pub struct Homepage;

impl BaseTemplatePage for Homepage {
    fn render_page(&self, sd: &SiteData, _si: &SiteIndex) -> (PageMeta, Markup) {
        let mut posts = sd.posts.iter().collect_vec();
        posts.sort_by_key(|p| p.meta().date.published);
        posts.reverse();
        posts.truncate(MAX_POSTS_ON_FRONT_PAGE);

        let content = html! {
            main .homepage .container {
                div style="text-align: center" {
                    img
                        src="https://s3.us-west-000.backblazeb2.com/nyaabucket/0aaa02e26cd9aee680f4ac3a2dc2f9c9e6792cdebcfc6d93255104e033de4654/under-construction.gif"
                        alt="under construction banner"
                        title="we are UNDER CONSTRUCTION!!!";
                }
                h1 { "welcome to the site" }
                p { "please enjoy the site" }
                (news_box(&sd.news))
                div .recent-posts-widget {
                    h2 { "Recent blog posts" }
                    (PostsTable { posts, tags: &sd.tags })
                }
                cat-chatbox { }
                (buttons(&sd.buttons))
            }
        };
        let meta = PageMeta {
            title: "Homepage".into(),
            navbar_highlighted: None,
            extra_head: html! {
                meta property="og:title" content="astrid dot tech";
                meta property="og:description" content="Astrid's personal website";
                meta property="og:type" content="website";
                meta property="og:url" content="https://astrid.tech";
            },
            ..Default::default()
        };

        (meta, content)
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
            img
                .clickable[b.onclick.is_some()]
                src=(b.img)
                width="88"
                height="31"
                alt=[&b.alt]
                title=[&b.title]
                onclick=[&b.onclick];
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
