use futures_channel::mpsc;

use lib_actor::ask;
use lib_protocol::core::{self, Assignment, ExperimentId, Report, RunnerId, Scenario};

use crate::backend::{ExperimentWatcher, Result, System};

pub use self::{
    actor::*,
    message::*,
};

mod actor;
mod message;

#[derive(Clone, Debug)]
pub struct Experiment {
    tx: ExperimentTx,
}

impl Experiment {
    pub fn spawn(system: System, id: ExperimentId, scenarios: Vec<Scenario>) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExperimentActor::new(
            rx,
            system,
            id,
            scenarios,
        ).start());

        Self { tx }
    }

    pub async fn as_model(&self) -> core::Experiment {
        ask!(self.tx, ExperimentMsg::AsModel)
    }

    pub async fn report(&self, runner: RunnerId, report: Report) -> Result<()> {
        ask!(self.tx, ExperimentMsg::Report { runner, report })
    }

    pub async fn start(&self, runner: RunnerId) -> Result<Assignment> {
        ask!(self.tx, ExperimentMsg::Start { runner })
    }

    pub async fn watch(&self) -> ExperimentWatcher {
        ask!(self.tx, ExperimentMsg::Watch)
    }
}