use derivative::Derivative;
use log::*;

use lib_core_actor::*;
use lib_core_channel::{OTx, SendTo};
use lib_interop::domain::DRunner;

use super::RunnerActor;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum RunnerMsg {
    GetModel {
        #[derivative(Debug = "ignore")]
        tx: OTx<DRunner>,
    },

    Kill,
}

mod get_model;

impl RunnerMsg {
    pub fn handle(self, actor: &mut RunnerActor) -> ActorWorkflow {
        debug!("Handling message: {:?}", self);

        match self {
            RunnerMsg::GetModel { tx } => {
                get_model::get_model(actor)
                    .send_to(tx);

                ActorWorkflow::Continue
            }

            RunnerMsg::Kill => {
                ActorWorkflow::Stop
            }
        }
    }
}