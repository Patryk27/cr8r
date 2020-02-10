use std::sync::Arc;

use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo, URx};
use lib_interop::domain::{DAssignment, DEvent, DExperiment, DReport, DRunnerId};

use super::ExperimentActor;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum ExperimentMsg {
    Abort,

    AddEvent {
        runner_id: DRunnerId,
        event: DEvent,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<()>>,
    },

    GetModel {
        #[derivative(Debug = "ignore")]
        tx: OTx<DExperiment>,
    },

    GetReports {
        #[derivative(Debug = "ignore")]
        tx: OTx<Vec<Arc<DReport>>>,
    },

    Start {
        runner_id: DRunnerId,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<DAssignment>>,
    },

    Watch {
        #[derivative(Debug = "ignore")]
        tx: OTx<Result<URx<Arc<DReport>>>>,
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
        trace!("Handling message: {:?}", self);

        match self {
            ExperimentMsg::Abort => {
                abort::abort(actor)
            }

            ExperimentMsg::AddEvent { runner_id, event, tx } => {
                add_event::add_event(actor, runner_id, event)
                    .send_to(tx)
            }

            ExperimentMsg::GetModel { tx } => {
                get_model::get_model(actor)
                    .send_to(tx)
            }

            ExperimentMsg::GetReports { tx } => {
                get_reports::get_reports(actor)
                    .send_to(tx)
            }

            ExperimentMsg::Start { runner_id, tx } => {
                start::start(actor, runner_id)
                    .send_to(tx)
            }

            ExperimentMsg::Watch { tx } => {
                watch::watch(actor)
                    .send_to(tx)
            }
        }
    }
}