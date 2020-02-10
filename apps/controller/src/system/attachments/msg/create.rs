use anyhow::*;

use lib_interop::domain::{DAttachmentId, DAttachmentName};
use lib_interop::proto::core::PAttachmentSize;

use crate::system::Attachment;

use super::super::{AttachmentsActor, AttachmentsError};

pub fn create(actor: &mut AttachmentsActor, name: DAttachmentName, size: PAttachmentSize) -> Result<DAttachmentId> {
    // @todo if there's not enough space, try GCing before failing

    ensure!(size <= actor.remaining_store_size, AttachmentsError::NotEnoughSpaceInStore {
        attachment_size: size,
        remaining_store_size: actor.remaining_store_size,
    });

    let id = actor.next_id.inc();

    actor.attachments.insert(id, Attachment::new(
        id,
        name,
        size,
    ));

    actor.remaining_store_size -= size;

    Ok(id)
}