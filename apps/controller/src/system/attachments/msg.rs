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
}

impl AttachmentsMsg {
    pub fn handle(self, actor: &mut AttachmentsActor) {
        trace!("Handling message: {:?}", self);

        match self {
            AttachmentsMsg::Create { name, size, tx } => {
                create::create(actor, name, size)
                    .send_to(tx);
            }

            AttachmentsMsg::Get { id, tx } => {
                get::get(actor, id)
                    .send_to(tx);
            }
        }
    }
}