use anyhow::*;

use lib_interop::domain::DAttachmentId;

use crate::system::Attachment;

use super::super::{AttachmentStoreActor, AttachmentStoreError};

pub fn find_one(actor: &AttachmentStoreActor, id: DAttachmentId) -> Result<Attachment> {
    actor.attachments
        .get(&id)
        .map(ToOwned::to_owned)
        .ok_or_else(|| AttachmentStoreError::AttachmentNotFound { id }.into())
}