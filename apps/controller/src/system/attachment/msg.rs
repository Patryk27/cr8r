use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo};
use lib_interop::domain::DAttachmentName;

use super::AttachmentActor;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum AttachmentMsg {
    GetName {
        tx: OTx<DAttachmentName>,
    },
}

impl AttachmentMsg {
    pub fn handle(self, actor: &mut AttachmentActor) {
        use AttachmentMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            GetName { tx } => {
                actor.name
                    .clone()
                    .send_to(tx);
            }
        }
    }
}