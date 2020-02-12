use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_actor::ActorWorkflow;
use lib_core_channel::{OTx, SendTo};
use lib_interop::domain::DAttachmentName;
use lib_interop::proto::core::PAttachmentSize;

use super::AttachmentActor;

mod add_chunk;
mod commit;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum AttachmentMsg {
    AddChunk {
        chunk: Vec<u8>,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<()>>,
    },

    Commit {
        #[derivative(Debug = "ignore")]
        tx: OTx<Result<()>>,
    },

    GetName {
        #[derivative(Debug = "ignore")]
        tx: OTx<DAttachmentName>,
    },

    GetSize {
        #[derivative(Debug = "ignore")]
        tx: OTx<PAttachmentSize>,
    },

    Kill,
}

impl AttachmentMsg {
    pub async fn handle(self, actor: &mut AttachmentActor) -> ActorWorkflow {
        use AttachmentMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            AddChunk { chunk, tx } => {
                add_chunk::add_chunk(actor, chunk)
                    .await
                    .send_to(tx);

                ActorWorkflow::Continue
            }

            Commit { tx } => {
                commit::commit(actor)
                    .await
                    .send_to(tx);

                ActorWorkflow::Continue
            }

            GetName { tx } => {
                actor.name
                    .clone()
                    .send_to(tx);

                ActorWorkflow::Continue
            }

            GetSize { tx } => {
                actor.size.send_to(tx);
                ActorWorkflow::Continue
            }

            Kill => {
                ActorWorkflow::Stop
            }
        }
    }
}