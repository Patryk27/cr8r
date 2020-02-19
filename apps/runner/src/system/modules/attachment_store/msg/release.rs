use log::*;

use lib_interop::models::DAttachmentId;

use super::super::AttachmentStoreActor;

pub async fn release(actor: &mut AttachmentStoreActor, id: DAttachmentId) {
    if !actor.attachments.remove(&id) {
        warn!("Tried to release a non-existing attachment [id={}]", id);
        return;
    }

    info!("Releasing attachment: id={}", id);

    // @todo
}