use itertools::Itertools;
use maud::{html, Markup, PreEscaped, Render};

use crate::{
    load::document::FullyLoadedDocument,
    model::{
        metadata::{Post, PostDates},
        site_data::{SiteData, SiteIndex, TagMap},
        tag::TaggableExt,
    },
    templates::util::{format_dt_html, tag_list},
};

use super::{util::format_dt, BaseTemplatePage, NavbarItem, PageMeta};

type DPost = FullyLoadedDocument<Post>;

#[derive(Clone)]
pub struct BlogIndexPage;

impl BaseTemplatePage for BlogIndexPage {
    fn render_page(&self, sd: &SiteData, _si: &SiteIndex) -> (PageMeta, Markup) {
        let mut posts = sd.posts.iter().collect_vec();
        posts.sort_by_key(|p| p.meta().date.published);
        posts.reverse();

        let content = html! {
            main .container .blog-root {
                h1 style="text-align: center;" { "Blog" }

                (PostsTable { posts, tags: &sd.tags })
            }
        };

        let page_meta = PageMeta {
            title: "Blog".into(),
            navbar_highlighted: NavbarItem::Blog.into(),
            extra_head: html! {
                meta property="og:title" content="Blog";
                meta property="og:description" content="List of blog articles";
                meta property="og:url" content="https://astrid.tech/blog";
                meta property="og:type" content="website";
            },
            ..Default::default()
        };

        (page_meta, content)
    }
}

#[derive(Clone)]
pub struct PostsTable<'a> {
    pub posts: Vec<&'a DPost>,
    pub tags: &'a TagMap,
}

impl Render for PostsTable<'_> {
    fn render(&self) -> Markup {
        html! {
            div .posts-table {
                @for p in &self.posts {
                    (RenderPost::from(*p).row(&self.tags))
                }
            }
        }
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
            div .post-row .nsfw[meta.has_tag("nsfw")] {
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
            nav
                .tile
                .nsfw[meta.has_tag("nsfw")]
                style=(format!("background-color: {}", meta.css_color()))
            {

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

    pub fn page_content(&self, tags: &TagMap) -> Markup {
        html! {
            article .post-content {
                header {
                    (self.title(false))
                    (self.tagline())
                    p .date { (self.date()) }
                    p { (tag_list(tags, &self.post.meta().tags)) }
                }

                (PreEscaped(&self.post.html()))
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

impl BaseTemplatePage for RenderPost<'_> {
    fn render_page(&self, sd: &SiteData, _si: &SiteIndex<'_>) -> (PageMeta, Markup) {
        let meta = PageMeta {
            title: self.post.meta().title.clone(),
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
