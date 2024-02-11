use maud::{html, Markup, PreEscaped, Render};

use crate::{
    load::document::FullyLoadedDocument,
    model::{
        metadata::{Post, PostDates},
        site_data::TagMap,
    },
    templates::util::{format_dt_html, tag_list, EmDash},
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
        let mut posts = self.posts.clone();
        posts.reverse();

        let content = html! {
            main .container .blog-root {
                h2 { "Blog" }

                div .posts-table {
                    @for p in posts {
                        (RenderPost::from(p).row(&self.tags))
                    }
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
    pub fn row(&self, tags: &TagMap) -> Markup {
        let meta = self.post.meta();

        html! {
            div .post-row {
                div .datepane {
                    p .date { a href=(meta.href()) { (self.date()) } }
                }
                div .itempane {
                    div .titlepane {
                        h2 .title { (self.linked_title()) }
                        @if let Some(t) = &meta.tagline {
                            p .tagline { (t) }
                        }
                    }
                    div .tagpane {
                        p .tags { (tag_list(tags, &self.post.meta().tags)) }
                    }
                }
            }
        }
    }

    pub fn tile(&self, tags: &TagMap) -> Markup {
        // TODO make summary
        let meta = self.post.meta();

        html! {
            nav .tile style=(format!("background-color: {}", meta.css_color())) {
                header {
                    h2 { (self.linked_title()) }
                    (self.tagline())
                }

                summary {
                    a .read-more href=(self.post.meta().href()) {
                        "Read more..."
                    }
                }

                footer {
                    (tag_list(tags, &meta.tags))
                    p { (self.date()) }
                }
            }
        }
    }

    pub fn linked_title(&self) -> Markup {
        let meta = self.post.meta();

        html! {
            a href=(meta.href()) { (meta.title) }
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
