use futures_channel::{mpsc, oneshot};

use lib_protocol::core::{self, Assignment, Report, RunnerId};

use crate::backend::{ExperimentWatcher, Result};

pub type ExperimentTx = mpsc::UnboundedSender<ExperimentMsg>;
pub type ExperimentRx = mpsc::UnboundedReceiver<ExperimentMsg>;

#[derive(Debug)]
pub enum ExperimentMsg {
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