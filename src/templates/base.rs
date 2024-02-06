use maud::{html, Markup, Render};

#[derive(Debug, Clone)]
pub struct Base {
    pub title: String,
    pub navbar: Navbar,
    pub content: Markup,
}

impl Render for Base {
    fn render(&self) -> Markup {
        html! {
            html {
                head {
                    title { (self.title) }
                    link rel="stylesheet" type="text/css" href="/style.css";
                }
                body {
                    (self.navbar)
                    (self.content)
                }
            }
        }
    }
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
                    li .active[self.highlighted == Some($item)] { ($item) }
                }
            };
        }

        html! {
            nav {
                ul {
                    (navbar_item!(NavbarItem::Homepage))
                    (navbar_item!(NavbarItem::Blog))
                    (navbar_item!(NavbarItem::Projects))
                    (navbar_item!(NavbarItem::About))
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
    Homepage,
    Blog,
    Projects,
    About,
}

impl NavbarItem {
    pub fn href(&self) -> &str {
        match self {
            NavbarItem::Homepage => "/",
            NavbarItem::Blog => "/blog",
            NavbarItem::Projects => "/projects",
            NavbarItem::About => "/about",
        }
    }

    pub fn text(&self) -> &str {
        match self {
            NavbarItem::Homepage => "astrid.tech",
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
