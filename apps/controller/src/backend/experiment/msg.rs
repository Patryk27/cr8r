use std::sync::Arc;

use derivative::Derivative;
use log::*;
use tokio::sync::{mpsc, oneshot};

use lib_interop::domain::{DAssignment, DEvent, DExperiment, DReport, DRunnerId};

use crate::backend::experiment::ExperimentActor;
use crate::backend::Result;

pub type ExperimentTx = mpsc::UnboundedSender<ExperimentMsg>;
pub type ExperimentRx = mpsc::UnboundedReceiver<ExperimentMsg>;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum ExperimentMsg {
    Abort,

    AddEvent {
        runner_id: DRunnerId,
        event: DEvent,
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<()>>,
    },

    GetModel {
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<DExperiment>,
    },

    GetReports {
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Vec<Arc<DReport>>>,
    },

    Start {
        runner_id: DRunnerId,
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<DAssignment>>,
    },

    Watch {
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<mpsc::UnboundedReceiver<Arc<DReport>>>>,
    },
}

mod abort;
mod add_event;
mod get_model;
mod get_reports;
mod start;
mod watch;

impl ExperimentMsg {
    pub fn handle(self, actor: &mut ExperimentActor) {
        debug!("Handling message: {:?}", self);

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