use futures_channel::{mpsc, oneshot};

use crate::backend::ExecutorStatus;

pub type ExperimentExecutorTx = mpsc::UnboundedSender<ExperimentExecutorMsg>;
pub type ExperimentExecutorRx = mpsc::UnboundedReceiver<ExperimentExecutorMsg>;

#[derive(Debug)]
pub enum ExperimentExecutorMsg {
    Status {
        tx: oneshot::Sender<ExecutorStatus>,
    }
}