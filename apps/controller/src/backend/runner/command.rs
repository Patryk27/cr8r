use futures_channel::{mpsc, oneshot};

use lib_protocol::core;

pub type RunnerCommandTx = mpsc::UnboundedSender<RunnerCommand>;
pub type RunnerCommandRx = mpsc::UnboundedReceiver<RunnerCommand>;

#[derive(Debug)]
pub enum RunnerCommand {
    AsModel {
        tx: oneshot::Sender<core::Runner>,
    },
}