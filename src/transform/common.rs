use vfs::VfsPath;

use crate::media::MediaRegistry;

pub struct TransformContext {
    content_root: VfsPath,
    media: MediaRegistry,
}

impl TransformContext {
    pub fn new(content_root: VfsPath, media: MediaRegistry) -> Self {
        Self {
            content_root,
            media,
        }
    }

    pub fn content_root(&self) -> &VfsPath {
        &self.content_root
    }

    pub fn media(&self) -> &MediaRegistry {
        &self.media
    }
}
