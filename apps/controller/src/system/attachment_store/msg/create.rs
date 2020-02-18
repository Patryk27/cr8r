use anyhow::*;
use tokio::fs::File;

use lib_interop::models::{DAttachmentId, DAttachmentName};
use lib_interop::proto::models::PAttachmentSize;

use crate::system::Attachment;

use super::super::{AttachmentStoreActor, AttachmentStoreError};

pub async fn create(actor: &mut AttachmentStoreActor, name: DAttachmentName, size: PAttachmentSize) -> Result<DAttachmentId> {
    // @todo if there's not enough space, try garbage-collecting attachments before failing

    ensure!(size <= actor.remaining_size, AttachmentStoreError::AttachmentTooLarge {
        attachment_size: size,
        remaining_store_size: actor.remaining_size,
    });

    let id = actor.next_id.inc();

    let path = actor.config
        .store_path
        .join(format!("{}", id));

    let file = File::create(&path)
        .await
        .with_context(|| format!("Could not create attachment's file: {}", path.display()))?;

    actor.attachments.insert(id, Attachment::new(
        id,
        name,
        size,
        path,
        file,
    ));

    actor.remaining_size -= size;

    Ok(id)
}