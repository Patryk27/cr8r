use anyhow::*;

use lib_interop::domain::DAttachmentId;

use crate::system::Attachment;

use super::super::{AttachmentsActor, AttachmentsError};

pub fn get(actor: &AttachmentsActor, id: DAttachmentId) -> Result<Attachment> {
    actor.attachments
        .get(&id)
        .map(ToOwned::to_owned)
        .ok_or_else(|| AttachmentsError::AttachmentNotFound { id }.into())
}