use chrono::{Datelike, Timelike};
use maud::{html, Markup};

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
