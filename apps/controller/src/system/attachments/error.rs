use thiserror::Error;

use lib_interop::domain::DAttachmentId;
use lib_interop::proto::core::PAttachmentSize;

#[derive(Error, Debug)]
pub enum AttachmentsError {
    #[error("Attachment is too big (store contains {remaining_store_size} bytes left, whereas your attachment has {attachment_size} bytes)")]
    NotEnoughSpaceInStore {
        attachment_size: PAttachmentSize,
        remaining_store_size: PAttachmentSize,
    },

    #[error("Attachment [id={id}] could not be found")]
    AttachmentNotFound {
        id: DAttachmentId,
    },
}
