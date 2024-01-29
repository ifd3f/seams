use std::borrow::Cow;

use gray_matter::{engine::YAML, Matter, Pod};
use serde::{de::DeserializeOwned, Deserialize};
use tracing::trace;
use vfs::{VfsError, VfsPath};

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

    /// Paths to assets that are associated with this document.
    pub assets: Vec<VfsPath>,
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
}

/// Errors regarding the document load phase.
#[derive(thiserror::Error, Debug)]
pub enum DocumentLoadError {
    #[error("Failed to parse document: {0}")]
    FailedToParse(String),

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
) -> Result<impl Iterator<Item = Result<Document<M>, DocumentLoadError>>, VfsError> {
    Ok(path.walk_dir()?.filter_map(|p| {
        let Ok(p) = p else { return None };

        match Document::load(p.clone()) {
            Ok(d) => Some(Ok(d)),
            Err(DocumentLoadError::UnrecognizedExtension(e)) => {
                trace!("skipping document due to unrecognized extension {e}");
                None
            }
            Err(e) => Some(Err((p, e))),
        }
    }))
}

pub fn fully_load_docdir<M: DeserializeOwned>(
    path: VfsPath,
) -> Result<impl Iterator<Item = Result<FullyLoadedDocument<M>, LoadError>>, VfsError> {
    let docs = load_docs_in_dir(path)?;
    Ok(docs.map(|d| d?.fully_load()))
}

pub struct FullyLoadedDocument<M> {
    pub document: Document<M>,
    pub content: Content,
    pub transformed: TransformedContent,
}

#[derive(thiserror::Error, Debug)]
pub enum ContentTransformError {}

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

    pub fn fully_load(self) -> Result<FullyLoadedDocument<M>, LoadError> {
        let content = self.content.load()?.into_owned();
        let transformed = content.transform()?;
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

/// Split a filename's last file extension off, returning both.
///
/// ```rust
/// assert_eq!(split_extension("porn.jpg", ("porn", "jpg")))
/// assert_eq!(split_extension("/foo/bar/spam.jpg", ("/foo/bar/spam", "jpg")))
/// assert_eq!(split_extension("no_extension", ("no_extension", "")))
/// ```
fn split_extension(pathname: &str) -> (&str, &str) {
    match pathname.rsplit_once('.') {
        Some((basename, ext)) => (basename, ext),
        None => (pathname, ""),
    }
}

impl ContentSource {
    pub fn load(&self) -> Result<Cow<'_, Content>, ContentLoadError> {
        use ContentLoadError::*;
        match self {
            ContentSource::Embedded(raw) => Ok(Cow::Borrowed(raw)),
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
}

impl Content {
    pub fn transform(&self) -> Result<TransformedContent, ContentTransformError> {
        match self.content_type {
            ContentType::Plaintext => Ok(TransformedContent {
                html: format!("<pre>{}</pre>", html_escape::encode_text(&self.raw)),
                assets: vec![],
            }),
            ContentType::Markdown => todo!(),
            ContentType::Html => Ok(TransformedContent {
                html: self.raw.clone(),
                assets: vec![],
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::*;
    use rstest::*;
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
