use futures_channel::{mpsc, oneshot};
use log::*;

use crate::backend::experiment_executor::ExperimentExecutorActor;
use crate::backend::ExperimentExecutorStatus;

pub type ExperimentExecutorTx = mpsc::UnboundedSender<ExperimentExecutorMsg>;
pub type ExperimentExecutorRx = mpsc::UnboundedReceiver<ExperimentExecutorMsg>;

#[derive(Debug)]
pub enum ExperimentExecutorMsg {
    GetStatus {
        tx: oneshot::Sender<ExperimentExecutorStatus>,
    }
}

mod get_status;

impl ExperimentExecutorMsg {
    pub fn process(self, actor: &mut ExperimentExecutorActor) {
        debug!("Processing message: {:?}", self);

        match self {
            ExperimentExecutorMsg::GetStatus { tx } => {
                let _ = tx.send(get_status::get_status(actor));
            }
        }
    }
}