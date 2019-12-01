use futures_channel::mpsc;

use lib_protocol::core::{Assignment, ExperimentId, Report, RunnerId, Scenario};

use crate::backend::{msg, Result, System};

pub use self::{
    actor::*,
    command::*,
};

mod actor;
mod command;

#[derive(Clone, Debug)]
pub struct Experiment {
    tx: ExperimentCommandTx,
}

impl Experiment {
    pub fn spawn(system: System, id: ExperimentId, scenarios: Vec<Scenario>) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExperimentActor::new(
            system,
            id,
            scenarios,
        ).start(rx));

        Self { tx }
    }

    pub async fn report(&self, runner: RunnerId, report: Report) -> Result<()> {
        msg!(self.tx, tx, ExperimentCommand::Report { runner, report, tx })
    }

    pub async fn start(&self, runner: RunnerId) -> Result<Assignment> {
        msg!(self.tx, tx, ExperimentCommand::Start { runner, tx })
    }
}