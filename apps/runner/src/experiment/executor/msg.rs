use derivative::Derivative;
use log::*;
use tokio::sync::{mpsc, oneshot};

use lib_core_actor::*;

use crate::experiment::ExperimentExecutorStatus;

use super::ExperimentExecutorActor;

pub type ExperimentExecutorTx = mpsc::UnboundedSender<ExperimentExecutorMsg>;
pub type ExperimentExecutorRx = mpsc::UnboundedReceiver<ExperimentExecutorMsg>;

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

        debug!("Handling message: {:?}", self);

        match self {
            Abort => {
                ActorWorkflow::Stop
            }

            GetStatus { tx } => {
                let _ = tx.send(actor.status);
                ActorWorkflow::Continue
            }
        }
    }
}