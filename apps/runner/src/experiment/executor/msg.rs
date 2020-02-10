use derivative::Derivative;
use log::*;
use tokio::sync::oneshot;

use lib_core_actor::*;
use lib_core_channel::SendTo;

use crate::experiment::ExperimentExecutorStatus;

use super::ExperimentExecutorActor;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum ExperimentExecutorMsg {
    Abort,

    GetStatus {
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<ExperimentExecutorStatus>,
    },
}

impl ExperimentExecutorMsg {
    pub fn handle(self, actor: &mut ExperimentExecutorActor) -> ActorWorkflow {
        use ExperimentExecutorMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            Abort => {
                ActorWorkflow::Stop
            }

            GetStatus { tx } => {
                actor.status.send_to(tx);
                ActorWorkflow::Continue
            }
        }
    }
}