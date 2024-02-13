use std::borrow::Cow;

use futures::{stream::FuturesUnordered, StreamExt};
use gray_matter::{engine::YAML, Matter, Pod};
use serde::de::DeserializeOwned;
use tracing::trace;
use vfs::{VfsError, VfsPath};

use crate::{
    errors::Errors,
    load::util::split_extension,
    media::MediaRegistry,
    model::site_data::SiteDataUserError,
    transform::{
        common::TransformContext,
        markdown::{transform_markdown, MarkdownError},
    },
};

/// A document that has metadata and a piece of content associated with it.
#[derive(Clone, Debug)]
pub struct Document<M> {
    /// Path the document is at.
    pub path: VfsPath,

    /// Metadata associated with the document.
    pub meta: M,
    pub content: ContentSource,
}

/// Content represented by raw backing data and a ContentType.
///
/// Content is something that can be transformed into HTML based on its type.
#[derive(Clone, Debug)]
pub struct Content {
    /// The file that this content came from
    pub path: VfsPath,

    /// Type to interpret the content as.
    pub content_type: ContentType,

    /// Raw data of the content.
    pub raw: String,
}

/// Where the content is relative to the meta file.
#[derive(Clone, Debug)]
pub enum ContentSource {
    /// Content was embedded inside the document file.
    Embedded(Content),

    /// Content is stored separately, and is expected to be at the given path.
    FileRef(VfsPath),
}

/// Content that has been transformed into HTML.
pub struct TransformedContent {
    /// The raw HTML.
    pub html: String,
}

/// A ContentType supported by this system.
#[derive(Clone, Copy, Debug)]
pub enum ContentType {
    Plaintext,
    Markdown,
    Html,
    // Jupyter,
}

/// Top-level errors.
#[derive(thiserror::Error, Debug)]
pub enum LoadError {
    #[error("Error during document load phase: {0}")]
    DocumentLoad(#[from] DocumentLoadError),

    #[error("Error during content load phase: {0}")]
    ContentLoad(#[from] ContentLoadError),

    #[error("Error during content transform phase: {0}")]
    ContentTransform(#[from] ContentTransformError),

    #[error("Error loading tags: {0}")]
    TagError(anyhow::Error),
}

/// Errors regarding the document load phase.
#[derive(thiserror::Error, Debug)]
pub enum DocumentLoadError {
    #[error("Markdown has no frontmatter")]
    MarkdownHasNoFrontmatter,

    #[error("Unrecognized file extension {0:?}")]
    UnrecognizedExtension(String),

    #[error("Filesystem error: {0}")]
    FsError(#[from] VfsError),

    #[error("Failed to parse object: {0}")]
    ParseObjectError(#[from] serde_json::Error),

    #[error("Failed to parse YAML: {0}")]
    ParseYamlError(#[from] serde_yaml::Error),

    #[error("Failed to parse TOML: {0}")]
    ParseTomlError(#[from] toml::de::Error),
}

impl DocumentLoadError {
    pub fn is_non_document(&self) -> bool {
        match self {
            DocumentLoadError::UnrecognizedExtension(_) => true,
            _ => false,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ContentLoadError {
    #[error("Unrecognized file extension {0:?}")]
    UnknownExtension(String),

    #[error("Filesystem error: {0}")]
    FsError(#[from] VfsError),
}

/// Recursively load all documents in directory.
fn load_docs_in_dir<M: DeserializeOwned>(
    path: VfsPath,
) -> Result<impl Iterator<Item = Result<Document<M>, (VfsPath, DocumentLoadError)>>, VfsError> {
    Ok(path.walk_dir()?.filter_map(|p| {
        let Ok(p) = p else { return None };

        let span = tracing::debug_span!("load_document", path = p.as_str());
        let _enter = span.enter();

        if !p.is_file().unwrap() {
            return None;
        };

        match Document::load(p.clone()) {
            Ok(d) => Some(Ok(d)),
            Err(e) if e.is_non_document() => {
                trace!("skipping non-document file: {e}");
                None
            }
            Err(e) => Some(Err((p, e))),
        }
    }))
}

/// Recursively load all the documents in a directory and their contents.
pub async fn fully_load_docdir<M: DeserializeOwned>(
    media: &MediaRegistry,
    path: VfsPath,
) -> Result<Vec<Result<FullyLoadedDocument<M>, SiteDataUserError>>, VfsError> {
    let docs = load_docs_in_dir(path)?;

    let futures = docs
        .map(|d| async move {
            let d = match d {
                Ok(d) => d,
                Err((p, e)) => {
                    return Err(SiteDataUserError {
                        path: p,
                        error: e.into(),
                    })
                }
            };
            let content_path = d.content.path();
            match d.fully_load_content(media).await {
                Ok(fld) => Ok(fld),
                Err(e) => Err(SiteDataUserError {
                    path: content_path,
                    error: e,
                }),
            }
        })
        .collect::<FuturesUnordered<_>>();

    Ok(futures.collect::<Vec<_>>().await)
}

pub struct FullyLoadedDocument<M> {
    pub document: Document<M>,
    pub content: Content,
    pub transformed: TransformedContent,
}

impl<M> FullyLoadedDocument<M> {
    pub fn meta(&self) -> &M {
        &self.document.meta
    }

    pub fn html(&self) -> &str {
        &self.transformed.html
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ContentTransformError {
    #[error("error: {0}")]
    Markdown(#[from] Errors<MarkdownError>),
}

impl<M> Document<M>
where
    M: DeserializeOwned,
{
    /// Load a [Document] from the given path.
    ///
    /// Does not load content.
    pub fn load(path: VfsPath) -> Result<Self, DocumentLoadError> {
        use DocumentLoadError::*;
        let (noext, ext) = split_extension(path.as_str());
        let noext = path.root().join(noext)?;

        let file_content = path.read_to_string()?;

        match ext.as_ref() {
            "md" | "markdown" => {
                let entity = Matter::<YAML>::new().parse(&file_content);
                let Some(frontmatter): Option<Pod> = entity.data else {
                    return Err(MarkdownHasNoFrontmatter);
                };
                let meta: M = frontmatter.deserialize()?;

                Ok(Self {
                    path: path.clone(),
                    meta,
                    content: ContentSource::Embedded(Content {
                        content_type: ContentType::Markdown,
                        raw: entity.content,
                        path,
                    }),
                })
            }

            "yml" | "yaml" => Ok(Self {
                path,
                meta: serde_yaml::from_str(&file_content)?,
                content: ContentSource::FileRef(noext),
            }),

            "toml" => Ok(Self {
                path,
                meta: toml::from_str(&file_content)?,
                content: ContentSource::FileRef(noext),
            }),

            e => Err(UnrecognizedExtension(e.into())),
        }
    }

    #[tracing::instrument(skip_all, fields(path = self.path.as_str()))]
    pub async fn fully_load_content(
        self,
        media: &MediaRegistry,
    ) -> Result<FullyLoadedDocument<M>, LoadError> {
        let content = self.content.load()?.into_owned();
        let transformed = content.transform(media).await?;

        Ok(FullyLoadedDocument {
            document: self,
            content,
            transformed,
        })
    }

    /// The root directory to consider assets from.
    pub fn asset_root(&self) -> VfsPath {
        self.path.parent()
    }
}

impl ContentSource {
    pub fn load(&self) -> Result<Cow<'_, Content>, ContentLoadError> {
        use ContentLoadError::*;

        match self {
            ContentSource::Embedded(c) => Ok(Cow::Borrowed(c)),
            ContentSource::FileRef(path) => {
                let Some(ext) = path.extension() else {
                    return Err(UnknownExtension("".into()));
                };
                let raw = path.read_to_string()?;

                let content_type = match ext.as_str() {
                    "html" => ContentType::Html,
                    "txt" => ContentType::Plaintext,
                    e => return Err(UnknownExtension(e.into())),
                };

                Ok(Cow::Owned(Content {
                    content_type,
                    raw,
                    path: path.clone(),
                }))
            }
        }
    }

    fn path(&self) -> VfsPath {
        match self {
            ContentSource::Embedded(c) => c.path.clone(),
            ContentSource::FileRef(p) => p.clone(),
        }
    }
}

impl Content {
    pub fn content_root(&self) -> VfsPath {
        self.path.parent()
    }

    #[tracing::instrument(skip_all, fields(ctype = ?self.content_type, path = self.path.as_str()))]
    pub async fn transform(
        &self,
        media: &MediaRegistry,
    ) -> Result<TransformedContent, ContentTransformError> {
        match self.content_type {
            ContentType::Plaintext => Ok(TransformedContent {
                html: format!("<pre>{}</pre>", html_escape::encode_text(&self.raw)),
            }),
            ContentType::Markdown => Ok(TransformedContent {
                html: transform_markdown(
                    &TransformContext::new(self.content_root(), media),
                    &self.raw,
                )
                .await?,
            }),
            ContentType::Html => Ok(TransformedContent {
                html: self.raw.clone(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::*;
    use rstest::*;
    use serde::Deserialize;
    use vfs::{PhysicalFS, VfsPath};

    #[derive(Deserialize)]
    struct Meta {
        field: String,
    }

    #[fixture]
    fn test_data() -> VfsPath {
        VfsPath::new(PhysicalFS::new("test_data"))
    }

    #[rstest]
    fn load_embedded_md(test_data: VfsPath) -> anyhow::Result<()> {
        let doc = Document::<Meta>::load(test_data.join("example_document.md")?)?;
        let content = doc.content.load()?;

        assert_eq!(doc.meta.field, "value");
        assert_matches!(doc.content, ContentSource::Embedded(_));
        assert!(
            content
                .raw
                .contains("Hello this is an example document bye"),
            "content.raw = {:?}",
            content.raw
        );

        Ok(())
    }

    #[rstest]
    fn load_separated_yml(test_data: VfsPath) -> anyhow::Result<()> {
        let doc = Document::<Meta>::load(test_data.join("my_yaml_doc.txt.yml")?)?;
        let content = doc.content.load()?;

        assert_eq!(doc.meta.field, "plain text");
        assert_matches!(doc.content, ContentSource::FileRef(_));
        assert!(
            content.raw.contains("i'm quite content"),
            "content.raw = {:?}",
            content.raw
        );

        Ok(())
    }

    #[rstest]
    fn fails_to_load_yaml_without_associated_content(test_data: VfsPath) -> anyhow::Result<()> {
        let doc = Document::<Meta>::load(
            test_data.join("nonexamples/yaml_without_associated_content.html.yml")?,
        )?;
        let content_result = doc.content.load();

        assert_matches!(doc.content, ContentSource::FileRef(_));
        assert_matches!(content_result, Err(_));

        Ok(())
    }
}
