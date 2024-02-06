use maud::{html, Markup, PreEscaped, Render};

use crate::{
    load::document::FullyLoadedDocument,
    model::metadata::{Post, PostDates},
    templates::util::format_date,
};

use super::{Base, Navbar, NavbarItem};

type DPost = FullyLoadedDocument<Post>;

#[derive(Clone)]
pub struct BlogIndexPage<'a> {
    pub posts: Vec<&'a DPost>,
}

impl Render for BlogIndexPage<'_> {
    fn render(&self) -> Markup {
        let content = html! {
            main .constrained {
                @for p in &self.posts {
                    (RenderPost::from(*p).short_item())
                }
            }
        };

        Base {
            title: "Blog".into(),
            navbar: NavbarItem::Blog.into(),
            content,
        }
        .render()
    }
}

#[derive(derive_more::From, Clone)]
pub struct RenderPost<'a> {
    #[from(forward)]
    post: &'a DPost,
}

impl<'a> RenderPost<'a> {
    pub fn short_item(&self) -> Markup {
        // TODO: fill in the summary
        html! {
            nav .post-preview {
                header {
                    (self.title(true))
                    (self.tagline())
                    p .date { (self.date()) }
                }

                summary {
                }

                p .read-more {
                    a href=(self.post.meta().href()) { "Read more..." }
                }
            }
        }
    }

    pub fn full_content_page(&self) -> Markup {
        Base {
            title: self.post.meta().title.clone(),
            navbar: NavbarItem::Blog.into(),
            content: html! {
                main .longform {
                    (self.page_content())
                }
            },
        }
        .render()
    }

    pub fn page_content(&self) -> Markup {
        html! {
            article .post-content {
                header {
                    (self.title(false))
                    (self.tagline())
                    p .date { (self.date()) }
                }

                (PreEscaped(&self.post.transformed.html))
            }
        }
    }

    fn title(&self, with_href: bool) -> Markup {
        let title = &self.post.meta().title;
        match with_href {
            true => html! {
                h1 .title { a href=(self.post.meta().href()) { (title) } }
            },
            false => html! {
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
}
