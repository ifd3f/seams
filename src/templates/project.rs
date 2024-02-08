use maud::{html, Markup, PreEscaped, Render};

use crate::{
    load::document::FullyLoadedDocument,
    model::{
        metadata::{Project, ProjectDates},
        site_data::TagMap,
    },
    templates::util::{format_dt_html, tag_list},
};

use super::{
    util::{format_dt, format_project_date},
    Base, NavbarItem,
};

type DProject = FullyLoadedDocument<Project>;

#[derive(Clone)]
pub struct ProjectIndexPage<'a> {
    pub projects: Vec<&'a DProject>,
    pub tags: &'a TagMap,
}

impl Render for ProjectIndexPage<'_> {
    fn render(&self) -> Markup {
        let mut projects = self.projects.clone();
        projects.reverse();

        let content = html! {
            main .container .projects-root {
                h1 { "Projects" }

                @for p in projects {
                    (RenderProject::from(p).short_item(&self.tags))
                }
            }
        };

        Base {
            title: "Broject".into(),
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
    pub fn short_item(&self, tags: &TagMap) -> Markup {
        // TODO: fill in the summary
        html! {
            nav .project-preview {
                header {
                    p { (self.date()) }
                    (self.title(true))
                    (self.tagline())
                    (tag_list(tags, &self.project.meta().tags))
                }

                summary {
                }

                p .read-more {
                    a href=(self.project.meta().href()) { "Read more..." }
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
