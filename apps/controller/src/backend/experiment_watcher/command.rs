use futures_channel::{mpsc, oneshot};

use lib_protocol::core::Report;

pub type ExperimentWatcherCommandTx = mpsc::UnboundedSender<ExperimentWatcherCommand>;
pub type ExperimentWatcherCommandRx = mpsc::UnboundedReceiver<ExperimentWatcherCommand>;

#[derive(Debug)]
pub enum ExperimentWatcherCommand {
    Add {
        report: Report,
    },

    Get {
        tx: oneshot::Sender<Option<Report>>,
    },

    Kill,
}
