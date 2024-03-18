use chrono::{Datelike, Month, Timelike};
use maud::{html, Markup, PreEscaped, Render};

use crate::model::{TagMap, TagSettings, TagStyling, Webring};

pub struct EmDash;

impl Render for EmDash {
    fn render(&self) -> Markup {
        PreEscaped("&mdash;".into())
    }
}

pub fn format_project_date(d: impl Datelike) -> String {
    let month = Month::try_from(d.month() as u8).unwrap();
    format!("{} {}", month.name(), d.year())
}

pub fn format_dt(d: impl Datelike + Timelike) -> String {
    format!(
        "{}-{:02}-{:02} {:02}:{:02}",
        d.year(),
        d.month(),
        d.day(),
        d.hour(),
        d.minute()
    )
}

pub fn format_dt_html(d: impl Datelike + Timelike) -> Markup {
    let dstr = format_dt(d);
    html! {
        time datetime=(&dstr) { (&dstr) }
    }
}

pub fn tag_list<I, S>(tag_map: &TagMap, tags: I) -> Markup
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    html! {
        span .tag-list {
            @for t in tags {
                (TagR::new(&tag_map[t.as_ref()])) " "
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TagR<'a> {
    settings: &'a TagSettings,
    link: bool,
}

impl<'a> TagR<'a> {
    pub fn new(settings: &'a TagSettings) -> Self {
        Self {
            settings,
            link: true,
        }
    }

    pub fn with_link(mut self, link: bool) -> Self {
        self.link = link;
        self
    }
}

impl Render for TagR<'_> {
    fn render(&self) -> Markup {
        macro_rules! style {
            ($text:expr, $bg:expr) => {
                format!("color: {}; background-color: {}", $text, $bg)
            };
        }

        let settings = self.settings;

        match (self.link, &settings.styling) {
            (true, TagStyling::Colors { text, bg }) => html! {
                a href=(settings.href) style=(style!(text, bg)) .tag {
                    (settings.title)
                }
            },
            (true, TagStyling::Class(c)) => html! {
                a href=(settings.href) .(c) .tag { (settings.title) }
            },
            (false, TagStyling::Colors { text, bg }) => html! {
                span style=(style!(text, bg)) .tag {
                    (settings.title)
                }
            },
            (false, TagStyling::Class(c)) => html! {
                span .(c) .tag { (settings.title) }
            },
        }
    }
}

#[derive(derive_more::From)]
pub struct RenderWebring<'a> {
    webring: &'a Webring,
}

impl Render for RenderWebring<'_> {
    fn render(&self) -> Markup {
        html! {
            p .webring {
                a .webring-prev href=(self.webring.prev) {"←"}
                " "
                (PreEscaped(&self.webring.html))
                " "
                a .webring-next href=(self.webring.next) {"→"}
            }
        }
    }
}
