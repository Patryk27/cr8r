use lib_interop::domain::attachment::DAttachment;

use super::super::AttachmentActor;

pub fn get_model(actor: &AttachmentActor) -> DAttachment {
    DAttachment {
        id: actor.id,
        name: actor.name.clone(),
        size: actor.size,
        created_at: actor.created_at.clone(),
    }
}