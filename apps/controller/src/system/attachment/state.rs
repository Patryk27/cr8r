use tokio::fs::File;

use lib_interop::proto::core::PAttachmentSize;

pub enum AttachmentState {
    Uninitialized,

    Pending {
        file: File,
        uploaded_bytes: PAttachmentSize,
    },

    Ready,
}

impl Default for AttachmentState {
    fn default() -> Self {
        AttachmentState::Uninitialized
    }
}