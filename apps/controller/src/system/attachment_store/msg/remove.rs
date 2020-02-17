use anyhow::*;

use lib_interop::domain::DAttachmentId;

use super::super::AttachmentStoreActor;

pub async fn remove(actor: &mut AttachmentStoreActor, id: DAttachmentId) -> Result<()> {
    let attachment = actor
        .attachments
        .remove(&id)
        .ok_or_else(|| anyhow!("Could not find attachment [id={}]", id))?;

    // @todo this `.get_model().await` may block the entire store (!) if some attachment gets stuck;
    //       eventually it'd be nice to keep a copy of attachment sizes locally (i.e. in this actor)

    let attachment_size = attachment
        .get_model()
        .await
        .size;

    actor.remaining_size += attachment_size;

    attachment.kill();

    Ok(())
}