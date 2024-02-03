use std::{borrow::Cow, io::Read};

use async_trait::async_trait;
use futures::AsyncRead;
use mime::Mime;
use sha2::{digest::crypto_common::KeyInit, Sha256};
use vfs::{
    async_vfs::{AsyncFileSystem, AsyncVfsPath}, AltrootFS, VfsPath
};

/// Some place that can receive media.
pub trait MediaRegistry {
    /// Register a piece of media to be uploaded, and returns a URL to where it should be
    /// when uploaded.
    ///
    /// This function does not have to upload media immediately. Requests may be batched
    /// later for more efficient upload.
    fn upload(&self, media: dyn Uploadable) -> anyhow::Result<String>;
}

/// Something that can be uploaded to a [MediaRegistry].
#[async_trait]
pub trait Uploadable {
    /// Filename associated with this file, if any.
    fn filename(&self) -> Option<String>;

    /// Media type be associated with this file, if any.
    fn mimetype(&self) -> Option<Mime>;

    /// The full content of this file.
    async fn body(&mut self) -> anyhow::Result<Cow<'_, [u8]>>;
}

#[async_trait]
impl Uploadable for VfsPath {
    fn filename(&self) -> Option<String> {
        Some(self.filename())
    }

    fn mimetype(&self) -> Option<Mime> {
        mime_guess::from_path(self.filename()).first()
    }

    async fn body(&mut self) -> anyhow::Result<Cow<'_, [u8]>> {
        let mut buf = vec![];
        self.open_file()?.read_to_end(&mut buf);
        Ok(Cow::Owned(buf))
    }
}

/// Upload from a slice in memory.
#[derive(Default)]
pub struct InMemoryUploadable<'a> {
    pub filename: Option<String>,
    pub mimetype: Option<Mime>,
    pub data: &'a [u8],
}

#[async_trait]
impl Uploadable for InMemoryUploadable<'_> {
    fn filename(&self) -> Option<String> {
        self.filename
    }

    fn mimetype(&self) -> Option<Mime> {
        self.mimetype
    }

    async fn body(&mut self) -> anyhow::Result<Cow<'_, [u8]>> {
        Ok(Cow::Borrowed(self.data))
    }
}

/// Upload from a file.
pub struct FileUploadable {
    pub filename: Option<String>,
    pub mimetype: Option<Mime>,
    pub path: VfsPath,
}

#[async_trait]
impl Uploadable for FileUploadable {
    fn filename(&self) -> Option<String> {
        self.filename
    }

    fn mimetype(&self) -> Option<Mime> {
        self.mimetype
    }

    async fn body(&mut self) -> anyhow::Result<Cow<'_, [u8]>> {
        self.path.body().await
    }
}

/// A [MediaRegistry] that stores uploaded media on a filesystem.
pub struct VfsMediaRegistry {
    /// URL prefix for every uploaded file
    prefix: String,

    /// Where to store the files
    storage_root: AltrootFS,

    /// List of all currently-uploaded files
    files: std::sync::Mutex<Vec<FileUploadable>>,
}

impl VfsMediaRegistry {
    /// Create a new VfsMediaRegistry.
    ///
    /// Arguments:
    /// - `prefix`: URL prefix for every uploaded file
    /// - `backing`: root directory for storing uploaded files
    pub fn new(prefix: String, backing: VfsPath) -> Self {
        Self {
            prefix,
            storage_root: AltrootFS::new(backing),
            files: Default::default(),
        }
    }

    /// Consume the VfsMediaRegistry, and returns a list of every file that has been stored.
    pub fn files(self) -> Vec<FileUploadable> {
        self.files.into_inner().unwrap()
    }
}

#[async_trait]
impl MediaRegistry for VfsMediaRegistry {
    async fn upload(&self, media: impl Uploadable) -> anyhow::Result<String> {
        let filename = media.filename();
        let mimetype = media.mimetype();
        let body= media.body().await?;
        let body = body.as_ref();

        let hasher = Sha256::new();
        hasher.update(body);
        let hash = hasher.finalize();
        let b16 = base16::encode_lower(&hash);

        let path = match filename {
            Some(n) => format!("{}/{}", b16, n),
            None => b16,
        };
        let storage_path = VfsPath::new(self.storage_root).join(&path)?;
        storage_path.parent().create_dir_all()?;
        storage_path.create_file()?.write_all(data);

        let fu = FileUploadable {
            filename,
            mimetype,
            path: storage_path,
        };

        self.files.lock().unwrap().push(fu);

        Ok(format!("{}/{}", self.prefix, path))
    }
}
