use std::sync::Arc;

use anyhow::*;
use tokio::sync::mpsc;

use lib_actor::{ask, tell};
use lib_interop::domain::{DAssignment, DEvent, DExperiment, DExperimentId, DJob, DReport, DRunnerId};

pub(self) use self::{
    actor::*,
    msg::*,
    status::*,
};

mod actor;
mod msg;
mod status;

#[derive(Clone, Debug)]
pub struct Experiment {
    tx: ExperimentTx,
}

impl Experiment {
    pub fn new(id: DExperimentId, jobs: Vec<DJob>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(ExperimentActor::new(
            rx,
            id,
            jobs,
        ).start());

        Self { tx }
    }

    // @todo there should be something like `DExperimentAbortReason`
    pub fn abort(&self) {
        tell!(self.tx, ExperimentMsg::Abort);
    }

    pub async fn add_event(&self, runner_id: DRunnerId, event: DEvent) -> Result<()> {
        ask!(self.tx, ExperimentMsg::AddEvent { runner_id, event })
    }

    pub async fn get_model(&self) -> DExperiment {
        ask!(self.tx, ExperimentMsg::GetModel)
    }

    pub async fn get_reports(&self) -> Vec<Arc<DReport>> {
        ask!(self.tx, ExperimentMsg::GetReports)
    }

    pub async fn start(&self, runner_id: DRunnerId) -> Result<DAssignment> {
        ask!(self.tx, ExperimentMsg::Start { runner_id })
    }

    pub async fn watch(&self) -> Result<mpsc::UnboundedReceiver<Arc<DReport>>> {
        ask!(self.tx, ExperimentMsg::Watch)
    }
}