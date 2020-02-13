use derivative::Derivative;
use log::*;

use lib_core_actor::*;
use lib_core_channel::{OTx, SendTo};

use crate::system::ExecutorStatus;

use super::ExecutorActor;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum ExecutorMsg {
    GetStatus {
        #[derivative(Debug = "ignore")]
        tx: OTx<ExecutorStatus>,
    },

    Stop,
}

impl ExecutorMsg {
    pub fn handle(self, actor: &mut ExecutorActor) -> ActorWorkflow {
        use ExecutorMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            GetStatus { tx } => {
                actor.status.send_to(tx);
                ActorWorkflow::Continue
            }

            Stop => {
                ActorWorkflow::Stop
            }
        }
    }
}