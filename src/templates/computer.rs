use itertools::Itertools;
use maud::{html, Markup, PreEscaped, Render};

use crate::{
    load::document::FullyLoadedDocument,
    model::{computers::Computer, SiteData, SiteIndex},
};

use super::{BaseTemplatePage, PageMeta};

type DComputer = FullyLoadedDocument<Computer>;

#[derive(Clone)]
pub struct ComputerIndexPage;

impl BaseTemplatePage for ComputerIndexPage {
    fn render_page(&self, sd: &SiteData, _si: &SiteIndex) -> (PageMeta, Markup) {
        let mut computers = sd.computers.iter().collect_vec();
        computers.sort_by(|a, b| Computer::display_ordering(b.meta(), a.meta()));

        let title = "Computers I have known and loved";

        let content = html! {
            main .container-md .blog-root {
                h1 style="text-align: center;" { (title) }

                (ComputersTable { computers })
            }
        };

        let page_meta = PageMeta {
            title: title.into(),
            href: "/computers".into(),
            navbar_path: vec!["computers".into()],
            extra_head: html! {
                meta property="og:title" content=(title);
                meta property="og:description" content="Nearly every physical computer I have owned";
                meta property="og:url" content="https://astrid.tech/computers";
                meta property="og:type" content="website";
            },
            ..Default::default()
        };

        (page_meta, content)
    }
}

#[derive(Clone)]
pub struct ComputersTable<'a> {
    pub computers: Vec<&'a DComputer>,
}

impl Render for ComputersTable<'_> {
    fn render(&self) -> Markup {
        html! {
            table .computer-index-table style="width: 100%" {
                thead {
                    th { "Acquired" }
                    th { "Decommissioned" }
                    th { "Name" }
                    th { "Type" }
                    th { "Status" }
                }
                tbody {
                    @for p in &self.computers {
                        (RenderComputer::from(*p).row())
                    }
                }
            }
        }
    }
}

#[derive(derive_more::From, Clone)]
pub struct RenderComputer<'a> {
    #[from(forward)]
    computer: &'a DComputer,
}

impl<'a> RenderComputer<'a> {
    pub fn row(&self) -> Markup {
        let meta = self.computer.meta();

        html! {
            tr {
                td { (meta.date.acquired.to_string()) }
                td {
                    @match meta.date.decomissioned {
                        Some(d) => (d),
                        _ => "—",
                    }
                }
                td { (self.linked_title()) }
                td { (meta.specs.r#type) }
                td { (meta.status) }
            }
        }
    }

    pub fn linked_title(&self) -> Markup {
        let meta = self.computer.meta();

        html! {
            a href=(meta.href()) { (meta.name) }
        }
    }

    pub fn page_content(&self) -> Markup {
        let meta = self.computer.meta();
        html! {
            article .post-content {
                header {
                    h1 { (meta.name) }
                    (computer_stat_table(meta))
                }

                (PreEscaped(&self.computer.html()))
            }
        }
    }
}

impl BaseTemplatePage for RenderComputer<'_> {
    fn render_page(&self, _sd: &SiteData, _si: &SiteIndex<'_>) -> (PageMeta, Markup) {
        let meta = self.computer.meta();
        let meta = PageMeta {
            title: meta.name.clone(),
            href: "/computers".into(),
            navbar_path: vec!["computers".into()],
            extra_head: html! {
                meta property="og:title" content=(meta.name);
                meta property="og:description" content=(format!("computer {}", meta.name));
                meta property="og:type" content="article";
                meta property="og:url" content=(format!("https://astrid.tech{}", meta.href()));
            },
            ..Default::default()
        };
        let content = html! {
            main .container-md .longform {
                (self.page_content())
            }
        };
        (meta, content)
    }
}

fn computer_stat_table(c: &Computer) -> impl Render {
    fn row(label: &str, data: impl Render) -> impl Render {
        html! {
            tr {
                th { (label) }
                td { (data) }
            }
        }
    }

    let s = &c.specs;

    html! {
        section .infobox {
            h1 { (c.name) }
            hr;

            table {
                tbody {
                    @if let Some(n) = &c.hostname {
                        (row("Hostname", n))
                    }
                    (row("Status", c.status.to_string()))
                    (row("Type", s.r#type.to_string()))
                    (row("Acquired", c.date.acquired.to_string()))
                    @if let Some(d) = &c.date.decomissioned {
                        (row("Decommissioned", d.to_string()))
                    }
                }
            }
            hr;

            h2 { "Stats" }
            table {
                tbody {
                    (row("Model", &s.model))
                    (row("CPU", &s.cpu))
                    @if let Some(g) = &s.gpu {
                        (row("GPU", g))
                    }
                    (row("RAM", &s.ram.size))
                }
            }
        }
    }
}
