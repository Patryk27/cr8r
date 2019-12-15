use std::sync::Arc;

use futures_channel::{mpsc, oneshot};
use log::*;

use lib_protocol::core::{PAssignment, PExperiment, PExperimentEvent, PExperimentReport, PRunnerId};

use crate::backend::experiment::ExperimentActor;
use crate::backend::Result;

pub type ExperimentTx = mpsc::UnboundedSender<ExperimentMsg>;
pub type ExperimentRx = mpsc::UnboundedReceiver<ExperimentMsg>;

#[derive(Debug)]
pub enum ExperimentMsg {
    Abort,

    AddEvent {
        runner: PRunnerId,
        event: PExperimentEvent,
        tx: oneshot::Sender<Result<()>>,
    },

    GetModel {
        tx: oneshot::Sender<PExperiment>,
    },

    GetReports {
        tx: oneshot::Sender<Vec<Arc<PExperimentReport>>>,
    },

    Start {
        runner: PRunnerId,
        tx: oneshot::Sender<Result<PAssignment>>,
    },

    Watch {
        tx: oneshot::Sender<Result<mpsc::UnboundedReceiver<Arc<PExperimentReport>>>>,
    },
}

mod abort;
mod add_event;
mod get_model;
mod get_reports;
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

            ExperimentMsg::AddEvent { runner, event, tx } => {
                let _ = tx.send(add_event::process(actor, runner, event));
            }

            ExperimentMsg::GetModel { tx } => {
                let _ = tx.send(get_model::process(actor));
            }

            ExperimentMsg::GetReports { tx } => {
                let _ = tx.send(get_reports::process(actor));
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