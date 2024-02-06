use maud::{html, Markup, Render};

use crate::templates::*;

pub struct Homepage {}

impl Render for Homepage {
    fn render(&self) -> Markup {
        let base = Base {
            title: "Homepage".into(),
            navbar: Navbar {
                highlighted: Some(NavbarItem::About),
            },
            content: html! {
                p { "welcome to my site nyaa" }
            },
        };

        base.render()
    }
}
