use std::io::Read;

use mime::Mime;
use sha2::{Digest, Sha256};
use vfs::{AltrootFS, VfsPath};

pub trait Uploadable {
    fn as_media(&self) -> anyhow::Result<Media>;
}

impl Uploadable for VfsPath {
    fn as_media(&self) -> anyhow::Result<Media> {
        let mut buf = vec![];
        self.open_file()?.read_to_end(&mut buf)?;

        Ok(Media {
            filename: Some(self.filename()),
            mimetype: mime_guess::from_path(self.filename()).first(),
            body: buf,
        })
    }
}

impl Uploadable for Media {
    fn as_media(&self) -> anyhow::Result<Media> {
        Ok(self.clone())
    }
}

/// Upload from a slice in memory.
#[derive(Clone)]
pub struct Media {
    pub filename: Option<String>,
    pub mimetype: Option<Mime>,
    pub body: Vec<u8>,
}

/// Upload from a slice in memory.
pub struct FileUploadable {
    pub filename: Option<String>,
    pub mimetype: Option<Mime>,
    pub path: VfsPath,
}

/// A [MediaRegistry] that stores uploaded media on a filesystem.
pub struct MediaRegistry {
    /// URL prefix for every uploaded file
    url_prefix: String,

    /// Where to store the files
    storage_root: VfsPath,

    /// List of all currently-uploaded files
    files: std::sync::Mutex<Vec<FileUploadable>>,
}

impl MediaRegistry {
    /// Create a new VfsMediaRegistry.
    ///
    /// Arguments:
    /// - `prefix`: URL prefix for every uploaded file
    /// - `backing`: root directory for storing uploaded files
    pub fn new(url_prefix: String, backing: VfsPath) -> Self {
        backing.create_dir_all().unwrap();
        Self {
            url_prefix,
            storage_root: VfsPath::new(AltrootFS::new(backing)),
            files: Default::default(),
        }
    }

    /// Consume the VfsMediaRegistry, and returns a list of every file that has been stored.
    pub fn into_files(self) -> Vec<FileUploadable> {
        self.files.into_inner().unwrap()
    }

    pub fn upload_media(&self, media: impl Uploadable) -> anyhow::Result<String> {
        let media = media.as_media()?;

        let mut hasher = Sha256::new();
        hasher.update(&media.body);
        let hash = hasher.finalize();
        let b16 = base16::encode_lower(&hash);

        let path = match &media.filename {
            Some(n) => format!("{}/{}", b16, n),
            None => b16,
        };
        let storage_path = self.storage_root.join(&path)?;
        storage_path.parent().create_dir_all()?;
        storage_path.create_file()?.write_all(&media.body)?;

        let fu = FileUploadable {
            filename: media.filename,
            mimetype: media.mimetype,
            path: storage_path,
        };

        self.files.lock().unwrap().push(fu);

        Ok(format!("{}/{}", self.url_prefix, path))
    }
}
