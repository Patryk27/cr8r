use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo};
use lib_interop::domain::DAttachmentId;

use crate::system::Attachment;

use super::AttachmentStoreActor;

mod download;
mod release;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum AttachmentStoreMsg {
    Download {
        id: DAttachmentId,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<Attachment>>,
    },

    Release {
        id: DAttachmentId,
    },
}

impl AttachmentStoreMsg {
    pub async fn handle(self, actor: &mut AttachmentStoreActor) {
        use AttachmentStoreMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            Download { id, tx } => {
                download::download(actor, id)
                    .await
                    .send_to(tx);
            }

            Release { id } => {
                release::release(actor, id)
                    .await;
            }
        }
    }
}