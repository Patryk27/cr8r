use std::path::{Path, PathBuf};

use lib_interop::domain::DAttachmentId;

use crate::system::AttachmentStore;

pub struct Attachment {
    id: DAttachmentId,
    path: PathBuf,
    store: AttachmentStore,
}

impl Attachment {
    pub fn new(id: DAttachmentId, path: PathBuf, store: AttachmentStore) -> Self {
        Self { id, path, store }
    }

    pub fn id(&self) -> DAttachmentId {
        self.id
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for Attachment {
    fn drop(&mut self) {
        self.store.release(self.id);
    }
}