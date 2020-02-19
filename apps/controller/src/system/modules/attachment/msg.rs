use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_actor::ActorWorkflow;
use lib_core_channel::{OTx, SendTo};
use lib_interop::models::attachment::DAttachment;

use super::{AttachmentActor, AttachmentDownloadToken};

mod commit;
mod download;
mod get_model;
mod upload_chunk;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum AttachmentMsg {
    Commit {
        #[derivative(Debug = "ignore")]
        tx: OTx<Result<()>>,
    },

    Download {
        #[derivative(Debug = "ignore")]
        tx: OTx<Result<AttachmentDownloadToken>>,
    },

    GetModel {
        #[derivative(Debug = "ignore")]
        tx: OTx<DAttachment>,
    },

    Kill,

    UploadChunk {
        chunk: Vec<u8>,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<()>>,
    },
}

impl AttachmentMsg {
    pub async fn handle(self, actor: &mut AttachmentActor) -> ActorWorkflow {
        use AttachmentMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            Commit { tx } => {
                commit::commit(actor).await
                    .send_to(tx);

                ActorWorkflow::Continue
            }

            Download { tx } => {
                download::download(actor)
                    .send_to(tx);

                ActorWorkflow::Continue
            }

            GetModel { tx } => {
                get_model::get_model(actor)
                    .send_to(tx);

                ActorWorkflow::Continue
            }

            Kill => {
                ActorWorkflow::Stop
            }

            UploadChunk { chunk, tx } => {
                upload_chunk::upload_chunk(actor, chunk).await
                    .send_to(tx);

                ActorWorkflow::Continue
            }
        }
    }
}