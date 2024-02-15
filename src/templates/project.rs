use maud::{html, Markup, PreEscaped, Render};

use crate::{
    load::document::FullyLoadedDocument, model::{
        metadata::{Project, ProjectDates},
        site_data::TagMap,
    }, random_coloring::{PASTEL, TINT}, templates::util::tag_list
};

use super::{util::format_project_date, Base, NavbarItem};

type DProject = FullyLoadedDocument<Project>;

#[derive(Clone)]
pub struct ProjectIndexPage<'a> {
    pub projects: Vec<&'a DProject>,
    pub tags: &'a TagMap,
}

impl Render for ProjectIndexPage<'_> {
    fn render(&self) -> Markup {
        let mut projects = self.projects.clone();
        projects.sort_by_key(|p| p.meta().date.sort_key());
        projects.reverse();

        let content = html! {
            header .container {
                h1 style="text-align: center" { "Projects" }
            }

            main .tile-container {
                @for p in projects {
                    (RenderProject::from(p).tile(&self.tags))
                }
            }
        };

        Base {
            title: "Project".into(),
            navbar: NavbarItem::Projects.into(),
            content,
        }
        .render()
    }
}

#[derive(derive_more::From, Clone)]
pub struct RenderProject<'a> {
    #[from(forward)]
    project: &'a DProject,
}

impl<'a> RenderProject<'a> {
    pub fn tile(&self, tags: &TagMap) -> Markup {
        let color = self.project.meta().css_color(TINT);
        let style = if let Some(url) = &self.project.meta().thumbnail {
            let rgba = format!("rgba({}, {}, {}, 0.1)", color.r, color.g, color.b,);
            format!(r#"background-image: linear-gradient({rgba}, {rgba}), url("{url}")"#,)
        } else {
            format!("background-color: {}", color.to_hex_string())
        };
        html! {
            nav .tile style=(style) {
                header {
                    h2 .title {
                        a href=(self.project.meta().href()) { (self.project.meta().title) }
                    }
                    (self.tagline())
                    (tag_list(tags, &self.project.meta().tags))
                    p .date { (self.date()) }
                }
            }
        }
    }

    pub fn full_content_page(&self, tags: &TagMap) -> Markup {
        Base {
            title: self.project.meta().title.clone(),
            navbar: NavbarItem::Projects.into(),
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
            article .project-content {
                header {
                    (self.title(false))
                    (self.tagline())
                    p .date { (self.date()) }
                    p { (tag_list(tags, &self.project.meta().tags)) }
                }

                (PreEscaped(&self.project.transformed.html))
            }
        }
    }

    fn title(&self, with_href: bool) -> Markup {
        let title = &self.project.meta().title;
        match with_href {
            true => html! {
                h1 .title { a href=(self.project.meta().href()) { (title) } }
            },
            false => html! {
                h1 .title { (title) }
            },
        }
    }

    fn tagline(&self) -> Markup {
        match &self.project.document.meta.tagline {
            Some(tagline) => html! {
                p .tagline { (tagline) }
            },
            None => html! {},
        }
    }

    fn date(&self) -> String {
        let ProjectDates {
            started, finished, ..
        } = &self.project.document.meta.date;

        let finished = match finished {
            Some(f) => format_project_date(*f),
            None => "Now".into(),
        };

        format!("{} - {}", format_project_date(*started), finished)
    }
}
