use chrono::{Datelike, Timelike};

pub fn format_date(d: impl Datelike + Timelike) -> String {
    format!(
        "{}-{:02}-{:02} {:02}:{:02}",
        d.year(),
        d.month(),
        d.day(),
        d.hour(),
        d.minute()
    )
}
