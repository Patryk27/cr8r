use anyhow::*;

use lib_interop::domain::DAttachmentId;

use super::super::AttachmentsActor;

pub async fn remove(actor: &mut AttachmentsActor, id: DAttachmentId) -> Result<()> {
    let attachment = actor
        .attachments
        .remove(&id)
        .ok_or_else(|| anyhow!("Could not find attachment [id={}]", id))?;

    let attachment_size = attachment
        .get_size()
        .await;

    actor.remaining_size += attachment_size;

    attachment.kill();

    Ok(())
}