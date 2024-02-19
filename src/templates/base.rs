use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

use crate::model::site_data::{SiteData, SiteIndex};

/// Renders pages using the base template
#[derive(Clone)]
pub struct BaseRenderer<'a> {
    pub script_templates: Vec<String>,
    pub site_data: &'a SiteData,
    pub site_index: &'a SiteIndex<'a>,
}

impl BaseRenderer<'_> {
    pub fn render_page(&self, page: impl BaseTemplatePage) -> Markup {
        let (page_meta, rendered) = page.render_page(self.site_data, self.site_index);
        let navbar = Navbar {
            highlighted: page_meta.navbar_highlighted,
        };

        html! {
            (DOCTYPE)
            html {
                head {
                    title { (page_meta.title) }
                    link rel="stylesheet" type="text/css" href="/styles.css";
                    script type="text/javascript" src="/bundle.js" {}
                }
                body {
                    (navbar)
                    (rendered)
                    div #script-templates style="display: none" {
                        @for st in &self.script_templates {
                            (PreEscaped(st))
                        }
                    }
                    footer {
                        div {
                            input #nsfw-switch type="checkbox";
                            label for="nsfw-switch" {
                                "I am over 18 and am willing to see Not Safe for Work (NSFW) content."
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct PageMeta {
    pub title: String,
    pub navbar_highlighted: Option<NavbarItem>,
}

/// A page that uses the base template.
pub trait BaseTemplatePage {
    fn render_page(&self, sd: &SiteData, si: &SiteIndex<'_>) -> (PageMeta, Markup);
}

#[derive(Debug, Clone)]
pub struct Navbar {
    pub highlighted: Option<NavbarItem>,
}

impl Render for Navbar {
    fn render(&self) -> Markup {
        macro_rules! navbar_item {
            ($item:expr) => {
                html! {
                    li .navitem .active[self.highlighted == Some($item)] { ($item) }
                }
            };
        }

        html! {
            header .site-heading {
                h1 .site-title {
                    a href="/" { "astrid dot tech" }
                }

                nav {
                    ul .navbar {
                        (navbar_item!(NavbarItem::Blog))
                        (navbar_item!(NavbarItem::Projects))
                        (navbar_item!(NavbarItem::About))
                    }
                }
            }
        }
    }
}

impl From<NavbarItem> for Navbar {
    fn from(highlighted: NavbarItem) -> Self {
        Self {
            highlighted: Some(highlighted),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavbarItem {
    Blog,
    Projects,
    About,
}

impl NavbarItem {
    pub fn href(&self) -> &str {
        match self {
            NavbarItem::Blog => "/blog",
            NavbarItem::Projects => "/projects",
            NavbarItem::About => "/about",
        }
    }

    pub fn text(&self) -> &str {
        match self {
            NavbarItem::Blog => "Blog",
            NavbarItem::Projects => "Projects",
            NavbarItem::About => "About",
        }
    }
}

impl Render for NavbarItem {
    fn render(&self) -> Markup {
        html! {
            a href=(self.href()) {
                (self.text())
            }
        }
    }
}
