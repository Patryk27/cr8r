use futures_channel::mpsc;

use lib_actor::tell;

use crate::core::ExperimentClient;

pub(self) use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct ExperimentJournalist {
    tx: ExperimentJournalistTx,
}

impl ExperimentJournalist {
    pub fn spawn(client: ExperimentClient) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExperimentJournalistActor::new(
            rx,
            client,
        ).main());

        Self { tx }
    }

    pub fn add_message(&self, message: impl Into<String>) {
        tell!(self.tx, ExperimentJournalistMsg::AddCustomMessage { message: message.into() });
    }

    pub fn add_process_output(&self, line: impl Into<String>) {
        tell!(self.tx, ExperimentJournalistMsg::AddProcessOutput { line: line.into() });
    }

    pub fn add_experiment_started(&self) {
        tell!(self.tx, ExperimentJournalistMsg::AddExperimentStarted);
    }

    pub fn add_experiment_completed(&self) {
        tell!(self.tx, ExperimentJournalistMsg::AddExperimentCompleted);
    }

    pub fn add_scenario_started(&self) {
        tell!(self.tx, ExperimentJournalistMsg::AddScenarioStarted);
    }

    pub fn add_scenario_completed(&self, success: bool) {
        tell!(self.tx, ExperimentJournalistMsg::AddScenarioCompleted { success });
    }
}