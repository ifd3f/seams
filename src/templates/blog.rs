use chrono::format::strftime;
use maud::{html, Markup, PreEscaped, Render};

use crate::{
    load::document::FullyLoadedDocument,
    model::metadata::{Post, PostDates},
    templates::util::format_date,
};

use super::{Base, Navbar, NavbarItem};

type DPost = FullyLoadedDocument<Post>;

pub struct BlogIndexPage<'a> {
    pub posts: Vec<RenderPost<'a>>,
}

impl Render for BlogIndexPage<'_> {
    fn render(&self) -> Markup {
        let content = html! {
            @for p in &self.posts {
                (p.short_item())
            }
        };

        Base {
            title: "Blog".into(),
            navbar: Navbar {
                highlighted: Some(NavbarItem::Blog),
            },
            content,
        }
        .render()
    }
}

#[derive(derive_more::From)]
pub struct RenderPost<'a> {
    #[from(forward)]
    post: &'a DPost,
}

impl<'a> RenderPost<'a> {
    pub fn short_item(&self) -> Markup {
        // TODO: summary
        html! {
            article .post-preview {
                (self.title(true))
                (self.tagline())
                p .date { (self.date()) }
            }
        }
    }

    pub fn page(&self) -> Markup {
        html! {
            article .post-content {
                (self.title(false))
                (self.tagline())
                p .date { (self.date()) }
                (PreEscaped(&self.post.transformed.html))
            }
        }
    }

    fn meta(&self) -> &Post {
        &self.post.document.meta
    }

    fn title(&self, with_href: bool) -> Markup {
        match (&self.meta().title, with_href) {
            (None, _) => html! { },
            (Some(title), true) => 
                html! {
                    h1 .title { a href=(self.meta().href()) { (title) } }
                },
            (Some(title), false) => html! {
                h1 .title { (title) }
            },
        }
    }

    fn tagline(&self) -> Markup {
        match &self.post.document.meta.tagline {
            Some(tagline) => html! {
                p .tagline { (tagline) }
            },
            None => html! {},
        }
    }

    fn date(&self) -> Markup {
        let PostDates {
            created: c,
            published: p,
            updated: u,
        } = &self.post.document.meta.date;
        let (c, p, u) = (format_date(*c), format_date(*p), u.map(format_date));
        match u {
            Some(u) => {
                let alt = format!("created: {c}\npublished: {p}\nlast updated: {u}");
                html! {
                    span title=(alt) { (format!("{p}*")) }
                }
            }
            None => {
                let alt = format!("created: {c}\npublished: {p}");
                html! {
                    span title=(alt) { (p) }
                }
            }
        }
    }

    fn metadata(&self) -> Markup {
        html! {}
    }
}
