use log::*;
use tokio::sync::{mpsc, oneshot};

use crate::backend::executor::ExecutorActor;
use crate::backend::ExecutorStatus;

pub type ExecutorTx = mpsc::UnboundedSender<ExecutorMsg>;
pub type ExecutorRx = mpsc::UnboundedReceiver<ExecutorMsg>;

#[derive(Debug)]
pub enum ExecutorMsg {
    GetStatus {
        tx: oneshot::Sender<ExecutorStatus>,
    }
}

mod get_status;

impl ExecutorMsg {
    pub fn handle(self, actor: &mut ExecutorActor) {
        debug!("Handling message: {:?}", self);

        match self {
            ExecutorMsg::GetStatus { tx } => {
                let _ = tx.send(get_status::get_status(actor));
            }
        }
    }
}