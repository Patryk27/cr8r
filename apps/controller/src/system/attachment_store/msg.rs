use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo};
use lib_interop::models::{DAttachmentId, DAttachmentName};
use lib_interop::proto::models::PAttachmentSize;

use crate::system::Attachment;

use super::AttachmentStoreActor;

mod create;
mod find_one;
mod remove;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum AttachmentStoreMsg {
    Create {
        name: DAttachmentName,
        size: PAttachmentSize,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<DAttachmentId>>,
    },

    FindOne {
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

impl AttachmentStoreMsg {
    pub async fn handle(self, actor: &mut AttachmentStoreActor) {
        use AttachmentStoreMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            Create { name, size, tx } => {
                create::create(actor, name, size)
                    .await
                    .send_to(tx);
            }

            FindOne { id, tx } => {
                find_one::find_one(actor, id)
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