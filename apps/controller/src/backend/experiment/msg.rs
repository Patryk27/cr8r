use std::sync::Arc;

use log::*;
use tokio::sync::{mpsc, oneshot};

use lib_interop::contract::{CAssignment, CExperiment, CExperimentEvent, CExperimentReport, CRunnerId};

use crate::backend::experiment::ExperimentActor;
use crate::backend::Result;

pub type ExperimentTx = mpsc::UnboundedSender<ExperimentMsg>;
pub type ExperimentRx = mpsc::UnboundedReceiver<ExperimentMsg>;

#[derive(Debug)]
pub enum ExperimentMsg {
    Abort,

    AddEvent {
        runner_id: CRunnerId,
        event: CExperimentEvent,
        tx: oneshot::Sender<Result<()>>,
    },

    GetModel {
        tx: oneshot::Sender<CExperiment>,
    },

    GetReports {
        tx: oneshot::Sender<Vec<Arc<CExperimentReport>>>,
    },

    Start {
        runner_id: CRunnerId,
        tx: oneshot::Sender<Result<CAssignment>>,
    },

    Watch {
        tx: oneshot::Sender<Result<mpsc::UnboundedReceiver<Arc<CExperimentReport>>>>,
    },
}

mod abort;
mod add_event;
mod get_model;
mod get_reports;
mod start;
mod watch;

impl ExperimentMsg {
    pub fn process(self, actor: &mut ExperimentActor) {
        debug!("Processing message: {:?}", self);

        match self {
            ExperimentMsg::Abort => {
                abort::abort(actor);
            }

            ExperimentMsg::AddEvent { runner_id: runner, event, tx } => {
                let _ = tx.send(add_event::add_event(actor, runner, event));
            }

            ExperimentMsg::GetModel { tx } => {
                let _ = tx.send(get_model::get_model(actor));
            }

            ExperimentMsg::GetReports { tx } => {
                let _ = tx.send(get_reports::get_reports(actor));
            }

            ExperimentMsg::Start { runner_id: runner, tx } => {
                let _ = tx.send(start::start(actor, runner));
            }

            ExperimentMsg::Watch { tx } => {
                let _ = tx.send(watch::watch(actor));
            }
        }
    }
}