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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NavbarItem {
    pub name: String,
    pub href: String,
    #[serde(
        default,
        deserialize_with = "crate::model::util::permissive_vec::deserialize"
    )]
    pub children: Vec<NavbarItem>,
}

impl NavbarItem {
    pub fn has_item(&self, href: &str) -> bool {
        // given href starts with our href
        path_has_prefix(href, &self.href) || self.children.iter().any(|c| c.has_item(href))
    }
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

/// Returns true if `prefix` is a URL parent of `url`.
fn path_has_prefix(url: &str, prefix: &str) -> bool {
    // given href starts with our href
    let mut needle = prefix.split('/');
    let mut haystack = url.split('/');

    loop {
        // invariant: needle and haystack have encountered the same items
        match (haystack.next(), needle.next()) {
            // same item: continue
            (Some(h), Some(n)) if n == h => continue,
            // needle is exhausted or both are exhausted: is prefix
            (_, None | Some("")) => return true,
            _ => return false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("/a/b/c/d", "/a", true)]
    #[case("/a/b/c/d", "/a/b/c/d", true)]
    #[case("/b/c/d", "/a/b/c/d", false)]
    #[case("/a/b/c/d", "/", true)]
    #[case("/abcd", "/ab", false)]
    fn test_path_has_prefix(#[case] url: &str, #[case] prefix: &str, #[case] expected: bool) {
        assert_eq!(path_has_prefix(url, prefix), expected);
    }
}
