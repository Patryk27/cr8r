use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct AttachmentDownloadToken {
    path: PathBuf,
    refcount: Arc<AtomicUsize>,
}

impl AttachmentDownloadToken {
    pub fn new(path: PathBuf, refcount: Arc<AtomicUsize>) -> Self {
        Self { path, refcount }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for AttachmentDownloadToken {
    fn drop(&mut self) {
        self.refcount.fetch_sub(1, Ordering::SeqCst);
    }
}