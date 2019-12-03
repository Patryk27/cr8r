use futures_channel::{mpsc, oneshot};

use lib_protocol::core::Report;

pub type ExperimentWatcherTx = mpsc::UnboundedSender<ExperimentWatcherMsg>;
pub type ExperimentWatcherRx = mpsc::UnboundedReceiver<ExperimentWatcherMsg>;

#[derive(Debug)]
pub enum ExperimentWatcherMsg {
    Add {
        report: Report,
    },

    Get {
        tx: oneshot::Sender<Option<String>>,
    },

    Kill,
}
