use futures_channel::{mpsc, oneshot};

use lib_protocol::core::{Assignment, RunnerId};

use crate::backend::Result;

pub type ExperimentCommandTx = mpsc::UnboundedSender<ExperimentCommand>;
pub type ExperimentCommandRx = mpsc::UnboundedReceiver<ExperimentCommand>;

#[derive(Debug)]
pub enum ExperimentCommand {
    Start {
        runner: RunnerId,
        tx: oneshot::Sender<Result<Assignment>>,
    },
}