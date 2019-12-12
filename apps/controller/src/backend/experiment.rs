use futures_channel::mpsc;

use lib_actor::ask;
use lib_protocol::core::{PAssignment, PExperiment, PExperimentId, PReport, PRunnerId, PScenario};

use crate::backend::{ExperimentWatcher, Result, System};

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
    pub fn spawn(system: System, id: PExperimentId, scenarios: Vec<PScenario>) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExperimentActor::new(
            rx,
            system,
            id,
            scenarios,
        ).main());

        Self { tx }
    }

    pub async fn add_report(&self, runner: PRunnerId, report: PReport) -> Result<()> {
        ask!(self.tx, ExperimentMsg::AddReport { runner, report })
    }

    pub async fn as_model(&self) -> PExperiment {
        ask!(self.tx, ExperimentMsg::AsModel)
    }

    pub async fn start(&self, runner: PRunnerId) -> Result<PAssignment> {
        ask!(self.tx, ExperimentMsg::Start { runner })
    }

    pub async fn watch(&self) -> ExperimentWatcher {
        ask!(self.tx, ExperimentMsg::Watch)
    }
}