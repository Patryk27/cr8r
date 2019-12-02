use futures_channel::{mpsc, oneshot};

use lib_protocol::core::{self, Assignment, Report, RunnerId};

use crate::backend::{ExperimentWatcher, Result};

pub type ExperimentCommandTx = mpsc::UnboundedSender<ExperimentCommand>;
pub type ExperimentCommandRx = mpsc::UnboundedReceiver<ExperimentCommand>;

#[derive(Debug)]
pub enum ExperimentCommand {
    AsModel {
        tx: oneshot::Sender<core::Experiment>,
    },

    Report {
        runner: RunnerId,
        report: Report,
        tx: oneshot::Sender<Result<()>>,
    },

    Start {
        runner: RunnerId,
        tx: oneshot::Sender<Result<Assignment>>,
    },

    Watch {
        tx: oneshot::Sender<ExperimentWatcher>,
    },
}