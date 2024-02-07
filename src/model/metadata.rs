use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

use crate::date_sort::DateSort;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    /// Title of the post.
    pub title: String,

    /// Tagline of the post.
    pub tagline: Option<String>,

    /// A short, URL-friendly string identifying this project.
    pub slug: String,

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
}

impl Post {
    pub fn href(&self) -> String {
        let slugday = &self.date.published;
        format!(
            "/{}/{:02}/{:02}/{}/{}",
            slugday.year(),
            slugday.month(),
            slugday.day(),
            0usize,
            &self.slug
        )
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
    pub title: Option<String>,

    /// Tagline of the project.
    pub tagline: Option<String>,

    /// A short, URL-friendly string identifying this project.
    pub slug: String,

    /// Tags associated with the post.
    pub tags: Vec<String>,

    /// Dates associated with the project.
    pub date: ProjectDates,
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
