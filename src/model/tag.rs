use std::collections::HashMap;

use frunk::{Monoid, Semigroup};
use serde::{Deserialize, Serialize};

pub type Color = String; // TODO change this

#[derive(Serialize, Deserialize, Clone)]
pub enum TagStyling {
    /// Style using colors.
    Colors { text: Color, bg: Color },

    /// Assign the tag a class in the stylesheet.
    Class(String),
}

/// Tag styles, fully materialized.
#[derive(Serialize, Deserialize, Clone)]
pub struct TagSettings {
    /// Name of the tag
    pub title: String,

    /// Where it links to
    pub href: String,

    pub styling: TagStyling,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TagSettingsDirectiveBody {
    pub title: Option<String>,
    pub text_color: Option<Color>,
    pub color: Option<Color>,
    pub class: Option<String>,
}

impl TagSettingsDirectiveBody {
    pub fn materialize(self, tag_slug: &str) -> TagSettings {
        let styling = match self.class {
            Some(c) => TagStyling::Class(c),
            None => TagStyling::Colors {
                text: self.text_color.unwrap_or("#ffffff".into()),
                bg: self.color.unwrap_or("#000000".into()),
            },
        };

        // TODO color random selection
        TagSettings {
            title: tag_slug.to_owned(),
            href: format!("/t/{tag_slug}"),
            styling,
        }
    }
}

impl Semigroup for TagSettingsDirectiveBody {
    fn combine(&self, other: &Self) -> Self {
        // other takes precedence over self
        Self {
            title: other.title.clone().or(self.title.clone()),
            text_color: other.text_color.clone().or(self.text_color.clone()),
            color: other.color.clone().or(self.color.clone()),
            class: other.class.clone().or(self.class.clone()),
        }
    }
}

impl Monoid for TagSettingsDirectiveBody {
    fn empty() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TagSettingsDirective {
    pub tags: Vec<String>,
    pub settings: TagSettingsDirectiveBody,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TagSettingsSheet(pub Vec<TagSettingsDirective>);

impl TagSettingsSheet {
    pub fn materialize(self, additional_tags: Vec<&str>) -> HashMap<String, TagSettings> {
        use std::collections::hash_map::Entry;

        let mut applied_directives = HashMap::<&str, TagSettingsDirectiveBody>::new();

        for d in &self.0 {
            for t in &d.tags {
                match applied_directives.entry(t.as_str()) {
                    Entry::Occupied(v) => {
                        *v.into_mut() = v.get().combine(&d.settings);
                    }
                    Entry::Vacant(v) => {
                        v.insert(d.settings.clone());
                    }
                }
            }
        }

        for t in additional_tags {
            applied_directives.entry(t).or_default();
        }

        applied_directives
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v.materialize(k)))
            .collect()
    }
}

impl Semigroup for TagSettingsSheet {
    fn combine(&self, other: &Self) -> Self {
        Self(self.0.combine(&other.0))
    }
}

impl Monoid for TagSettingsSheet {
    fn empty() -> Self {
        Default::default()
    }
}
