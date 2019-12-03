use futures_channel::{mpsc, oneshot};

use crate::backend::ExecutorStatus;

pub type ExecutorTx = mpsc::UnboundedSender<ExecutorMsg>;
pub type ExecutorRx = mpsc::UnboundedReceiver<ExecutorMsg>;

#[derive(Debug)]
pub enum ExecutorMsg {
    Status {
        tx: oneshot::Sender<ExecutorStatus>,
    }
}