use std::sync::Arc;

use anyhow::*;
use chrono::Utc;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::{URx, UTx};
use lib_interop::domain::{DEvent, DExperiment, DExperimentId, DJob, DReport, DRunnerId};

use self::{
    actor::*,
    msg::*,
    status::*,
};

mod actor;
mod msg;
mod status;

#[derive(Clone)]
pub struct Experiment {
    tx: UTx<ExperimentMsg>,
}

impl Experiment {
    pub fn new(id: DExperimentId, jobs: Vec<DJob>) -> Self {
        let (tx, rx) = unbounded_channel();

        spawn(ExperimentActor {
            id,
            jobs,
            created_at: Utc::now(),
            watchers: Default::default(),
            status: Default::default(),
        }.start(rx));

        Self { tx }
    }

    pub async fn add_event(&self, runner_id: DRunnerId, event: DEvent) -> Result<()> {
        ask!(self.tx, ExperimentMsg::AddEvent { runner_id, event })
    }

    pub async fn claim(&self, runner_id: DRunnerId) -> Result<()> {
        ask!(self.tx, ExperimentMsg::Claim { runner_id })
    }

    pub async fn get_model(&self) -> DExperiment {
        ask!(self.tx, ExperimentMsg::GetModel)
    }

    pub async fn get_reports(&self) -> Vec<Arc<DReport>> {
        ask!(self.tx, ExperimentMsg::GetReports)
    }

    pub fn stop(&self) {
        tell!(self.tx, ExperimentMsg::Stop);
    }

    pub async fn watch(&self) -> Result<URx<Arc<DReport>>> {
        ask!(self.tx, ExperimentMsg::Watch)
    }
}