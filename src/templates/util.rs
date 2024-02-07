use chrono::{Datelike, Timelike};
use maud::{html, Markup};

use crate::model::{
    site_data::TagMap,
    tag::{TagSettings, TagStyling},
};

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
        @for t in tags {
            (tag(&tag_map[t.as_ref()]))
        }
    }
}

pub fn tag(tag: &TagSettings) -> Markup {
    match &tag.styling {
        TagStyling::Colors { text, bg } => html! {
            a .tag href=(tag.href) style=(format!("color: {text}; background-color: {bg}")) {
                (tag.title)
            }
        },
        TagStyling::Class(c) => html! {
            a .tag href=(tag.href) .(c) { (tag.title) }
        },
    }
}
