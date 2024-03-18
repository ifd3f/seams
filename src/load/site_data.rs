use std::fmt::Display;

use frunk::Monoid;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use tracing::warn;
use vfs::{VfsError, VfsPath};

use crate::{
    errors::Errors,
    load::{
        document::{fully_load_docdir, FullyLoadedDocument, LoadError},
        settings::load_settings_in_dir,
    },
    media::MediaRegistry,
    model::{
        metadata::{ArbitraryPage, Post, Project},
        Button88x31, NavbarItem, NewsItem, SiteData, TagSettingsSheet, Webring,
    },
};

pub struct SiteDataLoader<'a> {
    path: VfsPath,
    media: &'a MediaRegistry,
    errors: tokio::sync::Mutex<Errors<SiteDataUserError>>,
}

#[derive(thiserror::Error, Debug)]
pub enum SiteDataLoadError {
    #[error("Filesystem error: {0}")]
    Vfs(#[from] VfsError),

    #[error("User errors occurred: {0}")]
    UserError(#[from] Errors<SiteDataUserError>),
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

impl<'a> SiteDataLoader<'a> {
    pub fn new(path: VfsPath, media: &'a MediaRegistry) -> Self {
        Self {
            path,
            media,
            errors: Errors::new().into(),
        }
    }

    pub async fn load(self) -> Result<SiteData, SiteDataLoadError> {
        macro_rules! parallel_run_and_unwrap {
            ($(let $var:ident = $e:expr;)*) => {
                let ($($var, )*) = tokio::join!($($e, )*);
                let errors = self.errors;
                errors.into_inner().into_result()?;
                let ($($var, )*) = ($($var?.unwrap(), )*);
            }
        }

        parallel_run_and_unwrap! {
            let posts = self.load_docdir::<Post>("blog");
            let projects = self.load_docdir::<Project>("projects");
            let pages = self.load_docdir::<ArbitraryPage>("pages");
            let tags = self.load_settings::<TagSettingsSheet>("tag");
            let news = self.load_settings::<Vec<NewsItem>>("news");
            let buttons = self.load_settings::<Vec<Button88x31>>("88x31");
            let webrings = self.load_settings::<Vec<Webring>>("webring");
            let navbar = self.load_settings::<Vec<NavbarItem>>("navbar");
        };

        let extra_head = match load_extra_head(&self.path) {
            Ok(h) => h,
            Err(e) => {
                warn!("Failed to load settings/extra_head.html, will not inject extra data into the <head>: {e}.");
                "".into()
            }
        };

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
        let tags = tags.materialize(additional_tags);

        Ok(SiteData {
            posts,
            projects,
            tags,
            pages,
            news,
            navbar,
            buttons,
            webrings,
            extra_head,
        })
    }

    async fn load_docdir<M: DeserializeOwned>(
        &self,
        dir: &str,
    ) -> Result<Option<Vec<FullyLoadedDocument<M>>>, SiteDataLoadError> {
        let path = self.path.join(dir)?;
        let (rs, errs): (Vec<FullyLoadedDocument<M>>, Vec<_>) =
            fully_load_docdir::<M>(self.media, path)
                .await?
                .into_iter()
                .partition_result();

        self.errors.lock().await.extend(errs);
        Ok(Some(rs))
    }

    async fn load_settings<T: DeserializeOwned + Monoid>(
        &self,
        ext: &str,
    ) -> Result<Option<T>, SiteDataLoadError> {
        let path = self.path.join("settings")?;
        Ok(match load_settings_in_dir::<T>(path.clone(), ext) {
            Ok(r) => Some(r),
            Err(e) => {
                self.errors.lock().await.push(SiteDataUserError {
                    path,
                    error: LoadError::SettingsError(e, ext.to_string()),
                });
                None
            }
        })
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
        let content_path = VfsPath::new(PhysicalFS::new("test_data/astrid_dot_tech_example"));
        remove_dir_all("./out").ok();
        create_dir_all("./out").unwrap();
        let out = VfsPath::new(PhysicalFS::new("./out"));

        let media = MediaRegistry::new("https://test".into(), out.join("static").unwrap());

        let _sd = SiteData::load(content_path, &media).await.unwrap();
    }
}
