use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo};
use lib_interop::domain::{DAttachmentId, DAttachmentName};
use lib_interop::proto::core::PAttachmentSize;

use crate::system::Attachment;

use super::AttachmentsActor;

mod create;
mod get;
mod remove;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum AttachmentsMsg {
    Create {
        name: DAttachmentName,
        size: PAttachmentSize,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<DAttachmentId>>,
    },

    Get {
        id: DAttachmentId,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<Attachment>>,
    },

    Remove {
        id: DAttachmentId,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<()>>,
    },
}

impl AttachmentsMsg {
    pub async fn handle(self, actor: &mut AttachmentsActor) {
        use AttachmentsMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            Create { name, size, tx } => {
                create::create(actor, name, size)
                    .await
                    .send_to(tx);
            }

            Get { id, tx } => {
                get::get(actor, id)
                    .send_to(tx);
            }

            Remove { id, tx } => {
                remove::remove(actor, id)
                    .await
                    .send_to(tx);
            }
        }
    }
}