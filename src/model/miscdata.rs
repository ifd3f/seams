use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// 88x31 button representation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Button88x31 {
    pub img: String,
    pub title: Option<String>,
    pub alt: Option<String>,
    pub href: Option<String>,
    pub onclick: Option<String>,
}

/// Webring representation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Webring {
    pub prev: String,
    pub next: String,
    pub html: String,
    pub pending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NewsItem {
    pub title: Option<String>,
    pub time: DateTime<FixedOffset>,
    pub content: String,
}

impl Ord for NewsItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for NewsItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
