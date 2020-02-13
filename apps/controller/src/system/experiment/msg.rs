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

    Stop,

    Watch {
        #[derivative(Debug = "ignore")]
        tx: OTx<Result<URx<Arc<DReport>>>>,
    },
}

mod add_event;
mod get_model;
mod get_reports;
mod start;
mod stop;
mod watch;

impl ExperimentMsg {
    pub fn handle(self, actor: &mut ExperimentActor) {
        use ExperimentMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            AddEvent { runner_id, event, tx } => {
                add_event::add_event(actor, runner_id, event)
                    .send_to(tx)
            }

            GetModel { tx } => {
                get_model::get_model(actor)
                    .send_to(tx)
            }

            GetReports { tx } => {
                get_reports::get_reports(actor)
                    .send_to(tx)
            }

            Start { runner_id, tx } => {
                start::start(actor, runner_id)
                    .send_to(tx)
            }

            Stop => {
                stop::stop(actor)
            }

            Watch { tx } => {
                watch::watch(actor)
                    .send_to(tx)
            }
        }
    }
}