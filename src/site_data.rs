use chrono::Datelike;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

use vfs::{VfsError, VfsPath};

use crate::{
    loading::{fully_load_docdir, FullyLoadedDocument, LoadError},
    media::MediaRegistry,
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
    pub async fn load(path: VfsPath, media: &MediaRegistry) -> Result<SiteData, SiteDataLoadError> {
        let (posts, projects) = tokio::join!(
            fully_load_docdir(media, path.join("blog")?),
            fully_load_docdir(media, path.join("projects")?)
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

    pub fn write_static_site(&self, outdir: VfsPath) -> anyhow::Result<()> {
        outdir.create_dir_all()?;

        for p in &self.posts {
            let slugday = p.document.meta.date.published;
            let postdir = outdir
                .join(format!(
                    "{}/{:02}/{:02}/{}/{}",
                    slugday.year(),
                    slugday.month(),
                    slugday.day(),
                    0usize,
                    &p.document.meta.slug
                ))
                .unwrap();
            postdir.create_dir_all()?;
            let mut out = postdir.join("index.html").unwrap().create_file()?;
            out.write_all(p.transformed.html.as_bytes())?;
        }

        let projectsdir = outdir.join("projects").unwrap();
        for p in &self.projects {
            let projectdir = outdir.join(&p.document.meta.slug)?;
            projectdir.create_dir_all()?;
            let mut out = projectdir.join("index.html").unwrap().create_file()?;
            out.write_all(p.transformed.html.as_bytes())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs::{create_dir_all, remove_dir_all};

    use vfs::{PhysicalFS, VfsPath};

    use crate::media::MediaRegistry;

    use super::SiteData;

    #[tokio::test]
    pub async fn loads_and_writes_example_content_dir_correctly() {
        let content_path = VfsPath::new(PhysicalFS::new("test_data/contentdir_example"));
        remove_dir_all("./out").ok();
        create_dir_all("./out").unwrap();
        let out = VfsPath::new(PhysicalFS::new("./out"));

        let media = MediaRegistry::new("https://test".into(), out.join("static").unwrap());

        let sd = SiteData::load(content_path, &media).await.unwrap();

        sd.write_static_site(out).unwrap();
    }
}
