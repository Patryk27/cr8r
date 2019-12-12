use futures_channel::{mpsc, oneshot};
use log::*;

use lib_protocol::core::{PAssignment, PExperiment, PReport, PRunnerId};

use crate::backend::{ExperimentWatcher, Result};
use crate::backend::experiment::ExperimentActor;

pub type ExperimentTx = mpsc::UnboundedSender<ExperimentMsg>;
pub type ExperimentRx = mpsc::UnboundedReceiver<ExperimentMsg>;

#[derive(Debug)]
pub enum ExperimentMsg {
    Abort,

    AddReport {
        runner: PRunnerId,
        report: PReport,
        tx: oneshot::Sender<Result<()>>,
    },

    AsModel {
        tx: oneshot::Sender<PExperiment>,
    },

    Start {
        runner: PRunnerId,
        tx: oneshot::Sender<Result<PAssignment>>,
    },

    Watch {
        tx: oneshot::Sender<ExperimentWatcher>,
    },
}

mod abort;
mod add_report;
mod as_model;
mod start;
mod watch;

// @todo use macro
impl ExperimentMsg {
    pub fn process(self, actor: &mut ExperimentActor) {
        debug!("Processing message: {:?}", self);

        match self {
            ExperimentMsg::Abort => {
                abort::process(actor);
            }

            ExperimentMsg::AddReport { runner, report, tx } => {
                let _ = tx.send(add_report::process(actor, runner, report));
            }

            ExperimentMsg::AsModel { tx } => {
                let _ = tx.send(as_model::process(actor));
            }

            ExperimentMsg::Start { runner, tx } => {
                let _ = tx.send(start::process(actor, runner));
            }

            ExperimentMsg::Watch { tx } => {
                let _ = tx.send(watch::process(actor));
            }
        }
    }
}