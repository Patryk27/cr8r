use std::path::PathBuf;

use thiserror::Error;

use lib_interop::models::DAttachmentId;
use lib_interop::proto::models::PAttachmentSize;

#[derive(Debug, Error)]
pub enum AttachmentStoreError {
    #[error("Attachment is too large (store contains {remaining_store_size} bytes left, whereas your attachment has {attachment_size} bytes)")]
    AttachmentTooLarge {
        attachment_size: PAttachmentSize,
        remaining_store_size: PAttachmentSize,
    },

    #[error("Attachment [id={id}] could not be found")]
    AttachmentNotFound {
        id: DAttachmentId,
    },

    #[error("Attachment's store does not exist: {path}")]
    StoreNotFound {
        path: PathBuf,
    },
}
