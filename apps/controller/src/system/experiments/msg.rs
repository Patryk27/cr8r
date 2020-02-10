use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo};
use lib_interop::domain::{DDefinition, DExperimentId};

use crate::system::Experiment;

use super::ExperimentsActor;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum ExperimentsMsg {
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
        tx: OTx<DExperimentId>,
    },
}

mod find_all;
mod find_one;
mod launch;

impl ExperimentsMsg {
    pub fn handle(self, actor: &mut ExperimentsActor) {
        trace!("Handling message: {:?}", self);

        match self {
            ExperimentsMsg::FindAll { tx } => {
                find_all::find_all(actor)
                    .send_to(tx);
            }

            ExperimentsMsg::FindOne { id, tx } => {
                find_one::find_one(actor, id)
                    .send_to(tx);
            }

            ExperimentsMsg::Launch { definition, tx } => {
                launch::launch(actor, definition)
                    .send_to(tx);
            }
        }
    }
}
