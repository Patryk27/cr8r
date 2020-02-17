use std::path::PathBuf;

use thiserror::Error;

use lib_interop::domain::DAttachmentId;

#[derive(Error, Debug)]
pub enum AttachmentStoreError {
    #[error("Attachment [id={id}] has not been yet released; this is most likely a bug in Runner")]
    AttachmentSlotTaken {
        id: DAttachmentId,
    },

    #[error("Attachment's store does not exist: {path}")]
    StoreNotFound {
        path: PathBuf,
    },
}