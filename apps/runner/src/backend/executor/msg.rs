use log::*;
use tokio::sync::{mpsc, oneshot};

use crate::backend::executor::ExperimentExecutorActor;
use crate::backend::ExecutorStatus;

pub type ExperimentExecutorTx = mpsc::UnboundedSender<ExperimentExecutorMsg>;
pub type ExperimentExecutorRx = mpsc::UnboundedReceiver<ExperimentExecutorMsg>;

#[derive(Debug)]
pub enum ExperimentExecutorMsg {
    GetStatus {
        tx: oneshot::Sender<ExecutorStatus>,
    }
}

mod get_status;

impl ExperimentExecutorMsg {
    pub fn handle(self, actor: &mut ExperimentExecutorActor) {
        debug!("Handling message: {:?}", self);

        match self {
            ExperimentExecutorMsg::GetStatus { tx } => {
                let _ = tx.send(get_status::get_status(actor));
            }
        }
    }
}