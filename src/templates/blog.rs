use maud::{html, Markup, PreEscaped, Render};

use crate::{
    load::document::FullyLoadedDocument,
    model::{
        metadata::{Post, PostDates},
        site_data::TagMap,
    },
    templates::util::{format_dt_html, tag_list},
};

use super::{util::format_dt, Base, NavbarItem};

type DPost = FullyLoadedDocument<Post>;

#[derive(Clone)]
pub struct BlogIndexPage<'a> {
    pub posts: Vec<&'a DPost>,
    pub tags: &'a TagMap,
}

impl Render for BlogIndexPage<'_> {
    fn render(&self) -> Markup {
        let content = html! {
            main .container .blog-root {
                @for p in &self.posts {
                    (RenderPost::from(*p).short_item(&self.tags))
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
    pub fn short_item(&self, tags: &TagMap) -> Markup {
        // TODO: fill in the summary
        html! {
            nav .post-preview {
                header {
                    p { (self.date()) }
                    (self.title(true))
                    (self.tagline())
                    (tag_list(tags, &self.post.meta().tags))
                }

                summary {
                }

                p .read-more {
                    a href=(self.post.meta().href()) { "Read more..." }
                }
            }
        }
    }

    pub fn full_content_page(&self, tags: &TagMap) -> Markup {
        Base {
            title: self.post.meta().title.clone(),
            navbar: NavbarItem::Blog.into(),
            content: html! {
                main .container .longform {
                    (self.page_content(tags))
                }
            },
        }
        .render()
    }

    pub fn page_content(&self, tags: &TagMap) -> Markup {
        html! {
            article .post-content {
                header {
                    (self.title(false))
                    (self.tagline())
                    p .date { (self.date()) }
                    p { (tag_list(tags, &self.post.meta().tags)) }
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
            created: c_raw,
            published: p_raw,
            updated: u_raw,
        } = &self.post.document.meta.date;
        let (c, p, u) = (format_dt(*c_raw), format_dt(*p_raw), u_raw.map(format_dt));
        match u {
            Some(u) => {
                let alt = format!("created: {c}\npublished: {p}\nlast updated: {u}");
                html! {
                    span .date title=(alt) { (format_dt_html(*p_raw)) (format!("*")) }
                }
            }
            None => {
                let alt = format!("created: {c}\npublished: {p}");
                html! {
                    span .date title=(alt) { (format_dt_html(*p_raw)) }
                }
            }
        }
    }
}
