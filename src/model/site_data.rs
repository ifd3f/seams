use std::collections::HashMap;

use vfs::VfsPath;

use crate::{
    load::{
        document::FullyLoadedDocument,
        site_data::{SiteDataLoadError, SiteDataLoader},
    },
    media::MediaRegistry,
    transform::statistics::count_swears,
};

use super::{
    metadata::{ArbitraryPage, Post, Project},
    tag::TagSettings,
    Button88x31, NavbarItem, NewsItem, Webring,
};

pub type TagMap = HashMap<String, TagSettings>;

#[derive(Default)]
pub struct SiteData {
    pub posts: Vec<FullyLoadedDocument<Post>>,
    pub projects: Vec<FullyLoadedDocument<Project>>,
    pub pages: Vec<FullyLoadedDocument<ArbitraryPage>>,
    pub tags: TagMap,
    pub news: Vec<NewsItem>,
    pub navbar: Vec<NavbarItem>,
    pub buttons: Vec<Button88x31>,
    pub webrings: Vec<Webring>,
    pub extra_head: String,
}

#[derive(Default, Clone)]
pub struct SiteIndex<'a> {
    pub tag_to_posts: HashMap<&'a str, Vec<&'a FullyLoadedDocument<Post>>>,
    pub tag_to_projects: HashMap<&'a str, Vec<&'a FullyLoadedDocument<Project>>>,
    pub swear_count: HashMap<&'a str, usize>,
}

impl SiteData {
    pub async fn load(path: VfsPath, media: &MediaRegistry) -> Result<SiteData, SiteDataLoadError> {
        SiteDataLoader::new(path, media).load().await
    }

    pub fn build_index(&self) -> SiteIndex<'_> {
        let mut out = SiteIndex::default();

        for (t, _) in &self.tags {
            out.tag_to_posts.insert(t, vec![]);
            out.tag_to_projects.insert(t, vec![]);
        }

        for p in &self.posts {
            for t in &p.meta().tags {
                out.tag_to_posts.entry(t.as_str()).or_default().push(p);
            }
            for (s, c) in count_swears(p.html()) {
                *out.swear_count.entry(s).or_default() += c;
            }
        }

        for p in &self.projects {
            for t in &p.meta().tags {
                out.tag_to_projects.entry(t.as_str()).or_default().push(p);
            }
            for (s, c) in count_swears(p.html()) {
                *out.swear_count.entry(s).or_default() += c;
            }
        }

        out
    }
}
