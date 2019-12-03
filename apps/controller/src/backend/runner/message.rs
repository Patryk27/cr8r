use futures_channel::{mpsc, oneshot};

use lib_protocol::core;

pub type RunnerTx = mpsc::UnboundedSender<RunnerMsg>;
pub type RunnerRx = mpsc::UnboundedReceiver<RunnerMsg>;

#[derive(Debug)]
pub enum RunnerMsg {
    AsModel {
        tx: oneshot::Sender<core::Runner>,
    },
}