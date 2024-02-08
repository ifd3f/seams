use maud::{html, Markup, Render};

use super::{Base, Navbar, NavbarItem};

pub struct AboutPage;

impl Render for AboutPage {
    fn render(&self) -> Markup {
        let base = Base {
            title: "About".into(),
            navbar: Navbar {
                highlighted: Some(NavbarItem::About),
            },
            content: html! {
                main .container {
                    p { "welcome to my about page" }
                }
            },
        };

        base.render()
    }
}
