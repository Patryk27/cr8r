use std::sync::Arc;

use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo, URx};
use lib_interop::models::{DEvent, DExperiment, DJob, DReport, DRunnerId};

use crate::system::Attachment;

use super::ExperimentActor;

mod add_event;
mod claim;
mod get_model;
mod get_reports;
mod stop;
mod watch;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum ExperimentMsg {
    AddEvent {
        runner_id: DRunnerId,
        event: DEvent,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<()>>,
    },

    Claim {
        runner_id: DRunnerId,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<()>>,
    },

    GetAttachments {
        #[derivative(Debug = "ignore")]
        tx: OTx<Vec<Attachment>>,
    },

    GetJobs {
        #[derivative(Debug = "ignore")]
        tx: OTx<Vec<DJob>>,
    },

    GetModel {
        #[derivative(Debug = "ignore")]
        tx: OTx<DExperiment>,
    },

    GetReports {
        #[derivative(Debug = "ignore")]
        tx: OTx<Vec<Arc<DReport>>>,
    },

    Stop,

    Watch {
        #[derivative(Debug = "ignore")]
        tx: OTx<Result<URx<Arc<DReport>>>>,
    },
}

impl ExperimentMsg {
    pub fn handle(self, actor: &mut ExperimentActor) {
        use ExperimentMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            AddEvent { runner_id, event, tx } => {
                add_event::add_event(actor, runner_id, event)
                    .send_to(tx);
            }

            Claim { runner_id, tx } => {
                claim::claim(actor, runner_id)
                    .send_to(tx);
            }

            GetAttachments { tx } => {
                actor.attachments
                    .clone()
                    .send_to(tx);
            }

            GetJobs { tx } => {
                let jobs: Vec<_> = actor.jobs
                    .values()
                    .cloned()
                    .collect();

                jobs.send_to(tx);
            }

            GetModel { tx } => {
                get_model::get_model(actor)
                    .send_to(tx);
            }

            GetReports { tx } => {
                get_reports::get_reports(actor)
                    .send_to(tx);
            }

            Stop => {
                stop::stop(actor);
            }

            Watch { tx } => {
                watch::watch(actor)
                    .send_to(tx);
            }
        }
    }
}