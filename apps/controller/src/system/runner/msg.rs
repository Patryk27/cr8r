use derivative::Derivative;
use log::*;

use lib_core_actor::*;
use lib_core_channel::{OTx, SendTo};
use lib_interop::models::DRunner;

use super::RunnerActor;

mod get_model;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum RunnerMsg {
    GetModel {
        #[derivative(Debug = "ignore")]
        tx: OTx<DRunner>,
    },

    Kill,
}

impl RunnerMsg {
    pub fn handle(self, actor: &mut RunnerActor) -> ActorWorkflow {
        use RunnerMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            GetModel { tx } => {
                get_model::get_model(actor)
                    .send_to(tx);

                ActorWorkflow::Continue
            }

            Kill => {
                ActorWorkflow::Stop
            }
        }
    }
}