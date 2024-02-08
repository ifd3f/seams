use itertools::Itertools;
use maud::{html, Markup, Render};

use crate::{
    date_sort::DateSort,
    load::document::FullyLoadedDocument,
    model::{
        metadata::{Post, Project},
        site_data::TagMap,
        tag::TagSettings,
    },
    templates::util,
};

use super::{Base, Navbar, RenderPost};

pub struct TagPage<'a> {
    pub slug: &'a str,
    pub settings: &'a TagSettings,
    pub posts: Vec<&'a FullyLoadedDocument<Post>>,
    pub projects: Vec<&'a FullyLoadedDocument<Project>>,
    pub all_tags: &'a TagMap,
}

impl TagPage<'_> {
    fn items(&self) -> Vec<TaggedItem<'_>> {
        let posts = self.posts.iter().copied().map(TaggedItem::Post);
        let projects = self.projects.iter().copied().map(TaggedItem::Project);
        let mut items = posts.chain(projects).collect_vec();
        items.sort_by_key(|i| i.sort_key());
        items.reverse();
        items
    }

    fn render_item(&self, item: TaggedItem) -> Markup {
        match item {
            TaggedItem::Post(p) => RenderPost::from(p).short_item(self.all_tags),
            TaggedItem::Project(_p) => html! {
                p { "TODO" }
            },
        }
    }
}

impl Render for TagPage<'_> {
    fn render(&self) -> Markup {
        let items = self.items();

        let content = html! {
            main .container {
                h1 { "Tag " (util::tag(self.settings)) }
                @for i in items {
                    (self.render_item(i))
                }
            }
        };

        Base {
            title: format!("Tag {}", self.settings.title),
            navbar: Navbar { highlighted: None },
            content,
        }
        .render()
    }
}

enum TaggedItem<'a> {
    Post(&'a FullyLoadedDocument<Post>),
    Project(&'a FullyLoadedDocument<Project>),
}

impl TaggedItem<'_> {
    fn sort_key(&self) -> DateSort {
        match self {
            TaggedItem::Post(p) => p.meta().date.created.into(),
            TaggedItem::Project(p) => p.meta().date.sort_key(),
        }
    }
}
