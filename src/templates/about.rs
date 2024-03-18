use maud::{html, Markup};

use crate::model::{SiteData, SiteIndex};

use super::{BaseTemplatePage, NavbarItem, PageMeta};

pub struct AboutPage;

impl BaseTemplatePage for AboutPage {
    fn render_page(&self, _sd: &SiteData, _si: &SiteIndex) -> (PageMeta, Markup) {
        let about = html! {
            h1 style="text-align: center" { "About" }

            p { "This website is where I write about my projects." }

            p { "I got into coding when I was around 12, back when I played a ton of Minecraft. There was this mod for the game called ComputerCraft that added computers to the game. I thought that was really cool, so I ended up teaching myself Lua to program those computers." }

            p { "Later on, I started branching out into more and more languages, technologies, and projects, and I eventually ended up with the almost-decade-long mess that you can see on the projects. I wanted a place to share what I've made, so I created this website." }

            p { "This website has undergone 2 rewrites. Now, it is under construction." }
        };

        let meta = PageMeta {
            title: "About".into(),
            navbar_highlighted: Some(NavbarItem::About),
            ..Default::default()
        };

        (
            meta,
            html! {
                main .container {
                    (about)
                }
            },
        )
    }
}
