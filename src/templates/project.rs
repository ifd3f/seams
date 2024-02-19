use maud::{html, Markup, PreEscaped};

use crate::{
    load::document::FullyLoadedDocument,
    model::{
        metadata::{Project, ProjectDates},
        site_data::{SiteData, SiteIndex, TagMap},
        tag::TaggableExt,
    },
    templates::util::tag_list,
};

use super::{util::format_project_date, BaseTemplatePage, NavbarItem, PageMeta};

type DProject = FullyLoadedDocument<Project>;

#[derive(Clone)]
pub struct ProjectIndexPage<'a> {
    pub projects: Vec<&'a DProject>,
    pub tags: &'a TagMap,
}

impl BaseTemplatePage for ProjectIndexPage<'_> {
    fn render_page(&self, _sd: &SiteData, _si: &SiteIndex<'_>) -> (PageMeta, Markup) {
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

        let meta = PageMeta {
            title: "Project".into(),
            navbar_highlighted: NavbarItem::Projects.into(),
        };

        (meta, content)
    }
}

#[derive(derive_more::From, Clone)]
pub struct RenderProject<'a> {
    #[from(forward)]
    project: &'a DProject,
}

impl<'a> RenderProject<'a> {
    pub fn tile(&self, tags: &TagMap) -> Markup {
        let meta = self.project.meta();

        html! {
            nav
                .tile
                .nsfw[meta.has_tag("nsfw")]
                style=(format!("background-color: {}", meta.css_color()))
            {
                header {
                    h2 .title {
                        a href=(meta.href()) { (meta.title) }
                    }
                    (self.tagline())
                    (tag_list(tags, &meta.tags))
                    p .date { (self.date()) }
                }
            }
        }
    }

    pub fn page_content(&self, tags: &TagMap) -> Markup {
        html! {
            article .project-content {
                header {
                    (self.title(false))
                    (self.tagline())
                    p .date { (self.date()) }
                    p { (tag_list(tags, &self.project.meta().tags)) }
                    @if self.project.meta().url.site.len() > 0 {
                        p { "Site: " }
                        ul {
                            @for link in &self.project.meta().url.site {
                                li { a href=(link) { (link) } }
                            }
                        }
                    }
                    @if self.project.meta().url.source.len() > 0 {
                        p { "Source: " }
                        ul {
                            @for link in &self.project.meta().url.source {
                                li { a href=(link) { (link) } }
                            }
                        }
                    }
                }

                (PreEscaped(&self.project.html()))
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

impl BaseTemplatePage for RenderProject<'_> {
    fn render_page(&self, sd: &SiteData, _si: &SiteIndex<'_>) -> (PageMeta, Markup) {
        let content = html! {
            main .container .longform {
                (self.page_content(&sd.tags))
            }
        };

        let meta = PageMeta {
            title: self.project.meta().title.clone(),
            navbar_highlighted: NavbarItem::Projects.into(),
        };

        (meta, content)
    }
}
