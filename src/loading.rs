use std::borrow::Cow;

use gray_matter::{engine::YAML, Matter, Pod};
use serde::{de::DeserializeOwned, Deserialize};
use vfs::{VfsError, VfsPath};

/// A document that has metadata and a piece of content associated with it.
#[derive(Clone, Debug)]
pub struct Document<M> {
    path: VfsPath,
    meta: M,
    content: ContentSource,
}

/// Content represented by raw backing data and a ContentType.
///
/// Content is something that can be transformed into HTML based on its type.
#[derive(Clone, Debug)]
pub struct Content {
    content_type: ContentType,
    raw: String,
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

/// Content that is not a document.
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
pub enum LoadContentError {
    #[error("Unrecognized file extension {0:?}")]
    UnknownExtension(String),

    #[error("Filesystem error: {0}")]
    FsError(#[from] VfsError),
}

#[derive(thiserror::Error, Debug)]
pub enum TransformContentError {}

impl<M> Document<M>
where
    M: DeserializeOwned,
{
    /// Load a [Document] from the given path.
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
                    path,
                    meta,
                    content: ContentSource::Embedded(Content {
                        content_type: ContentType::Markdown,
                        raw: entity.content,
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
    pub fn load(&self) -> Result<Cow<'_, Content>, LoadContentError> {
        use LoadContentError::*;
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

                Ok(Cow::Owned(Content { content_type, raw }))
            }
        }
    }
}

impl Content {
    pub fn transform(&self) -> Result<TransformedContent, TransformContentError> {
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
