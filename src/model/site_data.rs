use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use tracing::{trace, warn};
use vfs::{VfsError, VfsPath};

use crate::{
    errors::Errors,
    load::{
        document::{fully_load_docdir, FullyLoadedDocument, LoadError},
        settings::load_settings_in_dir,
    },
    media::MediaRegistry,
};

use super::{
    button88x31::Button88x31,
    metadata::{Post, Project},
    news::NewsItem,
    tag::{TagSettings, TagSettingsSheet},
    webring::Webring,
};

pub type TagMap = HashMap<String, TagSettings>;

pub struct SiteData {
    pub posts: Vec<FullyLoadedDocument<Post>>,
    pub projects: Vec<FullyLoadedDocument<Project>>,
    pub tags: TagMap,
    pub news: Vec<NewsItem>,
    pub buttons: Vec<Button88x31>,
    pub webrings: Vec<Webring>,
    pub extra_head: String,
}

#[derive(Default, Clone)]
pub struct SiteIndex<'a> {
    pub tag_to_posts: HashMap<&'a str, Vec<&'a FullyLoadedDocument<Post>>>,
    pub tag_to_projects: HashMap<&'a str, Vec<&'a FullyLoadedDocument<Project>>>,
}

#[derive(Debug)]
pub struct SiteDataUserError {
    pub path: VfsPath,
    pub error: LoadError,
}

impl Display for SiteDataUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "In file {}:\n  {}", self.path.as_str(), self.error)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SiteDataLoadError {
    #[error("Filesystem error: {0}")]
    Vfs(#[from] VfsError),

    #[error("User errors occurred: {0}")]
    UserError(#[from] Errors<SiteDataUserError>),
}

impl SiteData {
    pub async fn load(path: VfsPath, media: &MediaRegistry) -> Result<SiteData, SiteDataLoadError> {
        let (posts, projects) = tokio::join!(
            fully_load_docdir(media, path.join("blog")?),
            fully_load_docdir(media, path.join("projects")?)
        );

        let (mut posts, post_failures): (Vec<FullyLoadedDocument<Post>>, Vec<_>) =
            posts?.into_iter().partition_result();
        let (mut projects, project_failures): (Vec<FullyLoadedDocument<Project>>, Vec<_>) =
            projects?.into_iter().partition_result();

        posts.sort_by_key(|p| p.meta().date.published);
        projects.sort_by_key(|p| p.meta().date.sort_key());

        // this prevents move errors, albeit jankily.
        // TODO: get rid of this jank
        let (tags, tag_failure) =
            match load_settings_in_dir::<TagSettingsSheet>(path.join("settings")?, "tag") {
                Ok(r) => (Some(r), None),
                Err(e) => (None, Some(e)),
            };
        let (news, news_failure) =
            match load_settings_in_dir::<Vec<NewsItem>>(path.join("settings")?, "news") {
                Ok(r) => (Some(r), None),
                Err(e) => (None, Some(e)),
            };
        let (buttons, buttons_failure) =
            match load_settings_in_dir::<Vec<Button88x31>>(path.join("settings")?, "88x31") {
                Ok(r) => (Some(r), None),
                Err(e) => (None, Some(e)),
            };
        let (webrings, webrings_failure) =
            match load_settings_in_dir::<Vec<Webring>>(path.join("settings")?, "webring") {
                Ok(r) => (Some(r), None),
                Err(e) => (None, Some(e)),
            };

        let mut load_errors: Errors<SiteDataUserError> = Default::default();
        load_errors.extend(post_failures);
        load_errors.extend(project_failures);
        if let Some(e) = tag_failure {
            load_errors.push(SiteDataUserError {
                path: path.join("settings")?,
                error: LoadError::SettingsError(e),
            });
        }
        if let Some(e) = news_failure {
            load_errors.push(SiteDataUserError {
                path: path.join("settings")?,
                error: LoadError::SettingsError(e),
            });
        }
        if let Some(e) = buttons_failure {
            load_errors.push(SiteDataUserError {
                path: path.join("settings")?,
                error: LoadError::SettingsError(e),
            });
        }
        if let Some(e) = webrings_failure {
            load_errors.push(SiteDataUserError {
                path: path.join("settings")?,
                error: LoadError::SettingsError(e),
            });
        }

        let extra_head = match load_extra_head(&path) {
            Ok(h) => h,
            Err(e) => {
                warn!("Failed to load settings/extra_head.html, will not inject extra data into the <head>: {e}.");
                "".into()
            }
        };

        load_errors.into_result()?;

        let mut additional_tags: Vec<&str> = vec![];
        for p in &posts {
            for t in &p.meta().tags {
                additional_tags.push(t)
            }
        }
        for p in &projects {
            for t in &p.meta().tags {
                additional_tags.push(t)
            }
        }
        let tags = tags.unwrap().materialize(additional_tags);
        let news = news.unwrap();
        let buttons = buttons.unwrap();
        let webrings = webrings.unwrap();

        trace!(?tags, "finished loading");

        Ok(Self {
            posts,
            projects,
            tags,
            news,
            buttons,
            webrings,
            extra_head,
        })
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
        }

        for p in &self.projects {
            for t in &p.meta().tags {
                out.tag_to_projects.entry(t.as_str()).or_default().push(p);
            }
        }

        out
    }
}

fn load_extra_head(path: &VfsPath) -> anyhow::Result<String> {
    let mut buf = String::new();
    path.join("settings/head.html")?
        .open_file()?
        .read_to_string(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod test {
    use std::fs::{create_dir_all, remove_dir_all};

    use vfs::{PhysicalFS, VfsPath};

    use crate::media::MediaRegistry;

    use super::SiteData;

    #[tokio::test]
    pub async fn loads_example_content_dir_correctly() {
        let content_path = VfsPath::new(PhysicalFS::new("test_data/contentdir_example"));
        remove_dir_all("./out").ok();
        create_dir_all("./out").unwrap();
        let out = VfsPath::new(PhysicalFS::new("./out"));

        let media = MediaRegistry::new("https://test".into(), out.join("static").unwrap());

        let _sd = SiteData::load(content_path, &media).await.unwrap();
    }
}
