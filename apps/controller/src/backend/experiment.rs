use std::sync::Arc;

use futures_channel::mpsc;

use lib_actor::{ask, tell};
use lib_protocol::core::{PAssignment, PExperiment, PExperimentEvent, PExperimentId, PExperimentReport, PRunnerId, PScenario};

use crate::backend::Result;

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
    pub fn spawn(id: PExperimentId, scenarios: Vec<PScenario>) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExperimentActor::new(
            rx,
            id,
            scenarios,
        ).main());

        Self { tx }
    }

    // @todo there should be something like `PExperimentAbortReason`
    pub fn abort(&self) {
        tell!(self.tx, ExperimentMsg::Abort);
    }

    pub async fn add_event(&self, runner: PRunnerId, event: PExperimentEvent) -> Result<()> {
        ask!(self.tx, ExperimentMsg::AddEvent { runner, event })
    }

    pub async fn get_model(&self) -> PExperiment {
        ask!(self.tx, ExperimentMsg::GetModel)
    }

    pub async fn get_reports(&self) -> Vec<Arc<PExperimentReport>> {
        ask!(self.tx, ExperimentMsg::GetReports)
    }

    pub async fn start(&self, runner: PRunnerId) -> Result<PAssignment> {
        ask!(self.tx, ExperimentMsg::Start { runner })
    }

    pub async fn watch(&self) -> Result<mpsc::UnboundedReceiver<Arc<PExperimentReport>>> {
        ask!(self.tx, ExperimentMsg::Watch)
    }
}