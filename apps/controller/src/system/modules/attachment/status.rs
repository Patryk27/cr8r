use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use tokio::fs::File;

use lib_interop::proto::models::PAttachmentSize;

pub enum AttachmentStatus {
    Uninitialized,

    Pending {
        file: File,
        uploaded_bytes: PAttachmentSize,
    },

    Ready {
        active_download_tokens: Arc<AtomicUsize>,
    },
}

impl Default for AttachmentStatus {
    fn default() -> Self {
        AttachmentStatus::Uninitialized
    }
}