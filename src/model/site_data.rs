use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use tracing::trace;
use vfs::{VfsError, VfsPath};

use crate::{
    load::{
        document::{fully_load_docdir, FullyLoadedDocument, LoadError},
        settings::load_settings_in_dir,
    },
    media::MediaRegistry,
};

use super::{
    metadata::{Post, Project},
    tag::{TagSettings, TagSettingsSheet},
};

pub type TagMap = HashMap<String, TagSettings>;

pub struct SiteData {
    pub posts: Vec<FullyLoadedDocument<Post>>,
    pub projects: Vec<FullyLoadedDocument<Project>>,
    pub tags: TagMap,
}

pub struct SiteIndex {
    pub slug_to_posts: HashMap<String, ()>,
    pub projects: Vec<()>,
    pub tags: Vec<()>,
}

#[derive(Debug)]
pub struct SiteDataUserErrors {
    pub load_errors: Vec<(VfsPath, LoadError)>,
}

impl Display for SiteDataUserErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (path, error) in &self.load_errors {
            writeln!(f, "In file {}:\n  {}", path.as_str(), error)?;
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SiteDataLoadError {
    #[error("Filesystem error: {0}")]
    Vfs(#[from] VfsError),

    #[error("User errors occurred: {0}")]
    UserError(SiteDataUserErrors),
}

impl From<SiteDataUserErrors> for SiteDataLoadError {
    fn from(errs: SiteDataUserErrors) -> Self {
        Self::UserError(errs)
    }
}

impl SiteData {
    pub async fn load(path: VfsPath, media: &MediaRegistry) -> Result<SiteData, SiteDataLoadError> {
        let (posts, projects) = tokio::join!(
            fully_load_docdir(media, path.join("blog")?),
            fully_load_docdir(media, path.join("projects")?)
        );

        let (posts, post_failures): (Vec<FullyLoadedDocument<Post>>, Vec<_>) =
            posts?.into_iter().partition_result();
        let (projects, project_failures): (Vec<FullyLoadedDocument<Project>>, Vec<_>) =
            projects?.into_iter().partition_result();

        // this prevents move errors, albeit jankily.
        // TODO: get rid of this jank
        let (tags, tag_failure) =
            match load_settings_in_dir::<TagSettingsSheet>(path.join("settings")?, "tag") {
                Ok(r) => (Some(r), None),
                Err(e) => (None, Some(e)),
            };

        let mut load_errors = vec![];
        load_errors.extend(post_failures);
        load_errors.extend(project_failures);
        if let Some(e) = tag_failure {
            load_errors.push((path.join("settings")?, LoadError::TagError(e)));
        }

        if !load_errors.is_empty() {
            return Err(SiteDataUserErrors { load_errors })?;
        }

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

        trace!(?tags, "finished loading");

        Ok(Self {
            posts,
            projects,
            tags,
        })
    }

    fn build_index(&self) -> SiteIndex {
        todo!()
    }
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
