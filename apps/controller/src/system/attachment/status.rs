use tokio::fs::File;

use lib_interop::proto::models::PAttachmentSize;

pub enum AttachmentStatus {
    Uninitialized,

    Pending {
        file: File,
        uploaded_bytes: PAttachmentSize,
    },

    Ready,
}

impl Default for AttachmentStatus {
    fn default() -> Self {
        AttachmentStatus::Uninitialized
    }
}