use itertools::Itertools;
use maud::{html, Markup};

use crate::{
    date_sort::DateSort,
    load::document::FullyLoadedDocument,
    model::{
        metadata::{Post, Project},
        site_data::{SiteData, SiteIndex, TagMap},
        tag::TagSettings,
    },
    templates::util::TagR,
};

use super::{BaseTemplatePage, PageMeta, RenderPost, RenderProject};

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
            TaggedItem::Post(p) => RenderPost::from(p).tile(self.all_tags),
            TaggedItem::Project(p) => RenderProject::from(p).tile(self.all_tags),
        }
    }
}

impl BaseTemplatePage for TagPage<'_> {
    fn render_page(&self, _sd: &SiteData, _si: &SiteIndex<'_>) -> (PageMeta, Markup) {
        let items = self.items();

        let content = html! {
            header .container {
                h1 { "Tag " (TagR::new(self.settings).with_link(false)) }
            }

            main .tile-container {
                @for i in items {
                    (self.render_item(i))
                }
            }
        };

        let meta = PageMeta {
            title: format!("Tag {}", self.settings.title),
            navbar_highlighted: None,
        };
        (meta, content)
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
