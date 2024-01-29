use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};


use vfs::{VfsError, VfsPath};

use crate::{
    loading::{fully_load_docdir, FullyLoadedDocument, LoadError},
    metadata::{Post, Project},
};

pub struct SiteData {
    pub posts: Vec<FullyLoadedDocument<Post>>,
    pub projects: Vec<FullyLoadedDocument<Project>>,
    // pub tags: Vec<()>, // TODO
}

pub struct SiteIndex {
    slug_to_posts: HashMap<String, ()>,
    projects: Vec<()>,
    tags: Vec<()>,
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
    pub async fn load(path: VfsPath) -> Result<SiteData, SiteDataLoadError> {
        let (posts, projects) = tokio::join!(
            fully_load_docdir(path.join("blog")?),
            fully_load_docdir(path.join("projects")?)
        );

        let (posts, post_failures): (Vec<_>, Vec<_>) = posts?.into_iter().partition_result();
        let (projects, project_failures): (Vec<_>, Vec<_>) =
            projects?.into_iter().partition_result();

        let mut load_errors = vec![];
        load_errors.extend(post_failures);
        load_errors.extend(project_failures);

        if !load_errors.is_empty() {
            return Err(SiteDataUserErrors { load_errors })?;
        }

        Ok(Self { posts, projects })
    }
}