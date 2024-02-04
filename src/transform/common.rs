use vfs::VfsPath;

use crate::media::MediaRegistry;

pub struct TransformContext<'a> {
    content_root: VfsPath,
    media: &'a MediaRegistry,
}

impl<'a> TransformContext<'a> {
    pub fn new(content_root: VfsPath, media: &'a MediaRegistry) -> Self {
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
