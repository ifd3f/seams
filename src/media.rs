use async_trait::async_trait;
use mime::Mime;
use sha2::Sha256;
use tokio::io::AsyncRead;
use vfs::{
    async_vfs::{AsyncFileSystem, AsyncVfsPath},
    AltrootFS,
};

/// Some place that can receive media.
#[async_trait]
pub trait MediaRegistry {
    /// Register a piece of media to be uploaded, and returns a URL to where it should be
    /// when uploaded.
    ///
    /// This function does not have to upload media immediately. Requests may be batched
    /// later for more efficient upload.
    async fn upload(&self, media: impl Uploadable) -> anyhow::Result<String>;
}

/// Something that can be uploaded to a [MediaRegistry].
pub trait Uploadable {
    /// Filename associated with this file, if any.
    fn filename(&self) -> Option<String>;

    /// Media type be associated with this file, if any.
    fn mimetype(&self) -> Option<Mime>;

    /// Open the file for reading.
    fn open(&mut self) -> Box<dyn AsyncRead>;
}

impl Uploadable for AsyncVfsPath {
    fn filename(&self) -> Option<String> {
        Some(self.filename())
    }

    fn mimetype(&self) -> Option<Mime> {
        mime_guess::from_path(self.filename()).first()
    }

    fn open(&mut self) -> Box<dyn AsyncRead + '_> {
        self.open_file()
    }
}

/// Upload from a slice in memory.
#[derive(Default)]
pub struct InMemoryUploadable<'a> {
    pub filename: Option<String>,
    pub mimetype: Option<Mime>,
    pub data: &'a [u8],
}

impl Uploadable for InMemoryUploadable<'_> {
    fn filename(&self) -> Option<String> {
        self.filename
    }

    fn mimetype(&self) -> Option<Mime> {
        self.mimetype
    }

    fn open(&mut self) -> Box<dyn AsyncRead + '_> {
        Box::new(self.data)
    }
}

/// Upload from a file.
#[derive(Default)]
pub struct FileUploadable {
    pub filename: Option<String>,
    pub mimetype: Option<Mime>,
    pub path: AsyncVfsPath,
}

impl Uploadable for FileUploadable {
    fn filename(&self) -> Option<String> {
        self.filename
    }

    fn mimetype(&self) -> Option<Mime> {
        self.mimetype
    }

    fn open(&mut self) -> Box<dyn AsyncRead + '_> {
        self.path.open_file()
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
    pub fn new(prefix: String, backing: AsyncVfsPath) -> Self {
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

        let data = vec![];
        media.open().read_to_end(&mut data).await?;

        let hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let b16 = base16::encode_lower(&hash);

        let path = match filename {
            Some(n) => format!("{}/{}", b16, n),
            None => b16,
        };
        let storage_path = AsyncVfsPath::new(self.storage_root).join(&path)?;
        storage_path.parent().create_dir_all()?;
        storage_path.create_file().await?.write_all(data);

        let fu = FileUploadable {
            filename,
            mimetype,
            path,
        };

        self.files.lock().unwrap().push(fu);

        Ok(format!("{}/{}", self.prefix, path))
    }
}
