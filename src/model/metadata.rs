use chrono::{DateTime, Datelike, FixedOffset, NaiveDate};
use csscolorparser::Color;
use serde::{Deserialize, Serialize};

use crate::{
    date_sort::DateSort,
    random_coloring::{ColorProfileExt, PASTEL},
};

use super::tag::Taggable;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    /// Title of the post.
    pub title: String,

    /// Tagline of the post.
    pub tagline: Option<String>,

    /// A URL-friendly string identifying this post.
    pub slug: PostSlug,

    /// Dates important to the post.
    pub date: PostDates,

    /// Tags associated with the post.
    #[serde(default)]
    pub tags: Vec<String>,

    /// What client was used to write this post.
    pub client: Option<String>,

    /// What this post is in reply to, if anything at all.
    #[serde(default)]
    pub reply_to: Vec<String>,

    /// Where this post is associated with.
    pub location: Option<String>,

    /// Author information.
    pub author: Option<Author>,

    /// Accent color. If null, it will be randomly picked based on the slug.
    pub color: Option<Color>,
}

impl Post {
    pub fn href(&self) -> String {
        let slugday = self.slug.date.unwrap_or(self.date.published.date_naive());
        format!(
            "/{}/{:02}/{:02}/{}/{}",
            slugday.year(),
            slugday.month(),
            slugday.day(),
            self.slug.ordinal,
            &self.slug.name
        )
    }

    pub fn css_color(&self) -> String {
        extract_color(self.color.clone(), &self.slug.name)
    }
}

impl Taggable for Post {
    fn tags(&self) -> impl Iterator<Item = &str> {
        self.tags.iter().map(|s| s.as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostDates {
    /// When this document was completed, but not necessarily published. However,
    /// it is usually the same date as the publish date.
    pub created: DateTime<FixedOffset>,

    /// When this document was published.
    pub published: DateTime<FixedOffset>,

    /// When this document was last updated.
    pub updated: Option<DateTime<FixedOffset>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    /// Name of the author.
    pub name: String,

    /// Short biography of the author.
    pub biography: String,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    /// Title of the project.
    pub title: String,

    /// Tagline of the project.
    pub tagline: Option<String>,

    /// A short, URL-friendly string identifying this project.
    pub slug: String,

    /// Tags associated with the post.
    pub tags: Vec<String>,

    /// Dates associated with the project.
    pub date: ProjectDates,

    /// URLs associated with the project.
    #[serde(default)]
    pub url: ProjectUrls,

    /// Accent color. If null, it will be randomly picked based on the slug.
    pub color: Option<Color>,
}

/// A generic page.
#[derive(Serialize, Deserialize)]
pub struct ArbitraryPage {
    /// Title of the page.
    pub title: String,

    /// Tags associated with the page.
    pub tags: Vec<String>,

    /// The URL path this page should have.
    pub slug: Vec<String>,

    /// Navbar ID path to highlight when this page is visited, or null
    /// to not highlight anything
    pub navbar_path: Option<Vec<String>>,

    /// Accent color. If null, it will be randomly picked based on the slug.
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostSlug {
    pub date: Option<NaiveDate>,
    #[serde(default)]
    pub ordinal: u8,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectUrls {
    #[serde(
        default,
        deserialize_with = "crate::model::util::permissive_vec::deserialize"
    )]
    pub site: Vec<String>,

    #[serde(
        default,
        deserialize_with = "crate::model::util::permissive_vec::deserialize"
    )]
    pub source: Vec<String>,
}

impl Project {
    pub fn href(&self) -> String {
        format!("/projects/{}", self.slug)
    }

    pub fn css_color(&self) -> String {
        extract_color(self.color.clone(), &self.slug)
    }
}

impl Taggable for Project {
    fn tags(&self) -> impl Iterator<Item = &str> {
        self.tags.iter().map(|s| s.as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectDates {
    /// When the project was started.
    pub started: NaiveDate,

    /// When the project was finished. Leave blank if unfinished.
    pub finished: Option<NaiveDate>,

    /// What date to sort this project by.
    ///
    /// If not provided, use the avg between start and finished.
    pub sort_date: Option<NaiveDate>,

    /// When this project page was published.
    pub published: Option<DateTime<FixedOffset>>,
}

impl ProjectDates {
    pub fn sort_key(&self) -> DateSort {
        if let Some(sd) = &self.sort_date {
            return (*sd).into();
        }

        if let Some(f) = self.finished {
            return f.into();
        };

        return DateSort::Now;
    }
}

impl ArbitraryPage {
    pub fn css_color(&self) -> String {
        extract_color(self.color.clone(), &self.slug.join("/"))
    }
}

fn extract_color(color: Option<Color>, slug: &str) -> String {
    if let Some(c) = color {
        return c.to_hex_string();
    }
    let color = PASTEL.for_text(slug);
    Color::from((color.red, color.green, color.blue)).to_hex_string()
}
