use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo};
use lib_interop::models::{DDefinition, DExperimentId, DRunnerId};

use crate::system::Experiment;

use super::ExperimentStoreActor;

mod delete;
mod find_all;
mod find_one;
mod launch;
mod prepare_assignment;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum ExperimentStoreMsg {
    Delete {
        id: DExperimentId,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<()>>,
    },

    FindAll {
        #[derivative(Debug = "ignore")]
        tx: OTx<Vec<Experiment>>,
    },

    FindOne {
        id: DExperimentId,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<Experiment>>,
    },

    Launch {
        definition: DDefinition,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<DExperimentId>>,
    },

    PrepareAssignment {
        runner_id: DRunnerId,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<Option<DExperimentId>>>,
    },
}

impl ExperimentStoreMsg {
    pub async fn handle(self, actor: &mut ExperimentStoreActor) {
        use ExperimentStoreMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            Delete { id, tx } => {
                delete::delete(actor, id)
                    .send_to(tx);
            }

            FindAll { tx } => {
                find_all::find_all(actor)
                    .send_to(tx);
            }

            FindOne { id, tx } => {
                find_one::find_one(actor, id)
                    .send_to(tx);
            }

            Launch { definition, tx } => {
                launch::launch(actor, definition).await
                    .send_to(tx);
            }

            PrepareAssignment { runner_id, tx } => {
                prepare_assignment::prepare_assignment(actor, runner_id).await
                    .send_to(tx);
            }
        }
    }
}
