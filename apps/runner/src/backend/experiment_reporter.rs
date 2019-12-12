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
pub struct ExperimentReporter {
    tx: ExperimentReporterTx,
}

impl ExperimentReporter {
    pub fn spawn(client: ExperimentClient) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExecutorReporterActor::new(
            rx,
            client,
        ).main());

        Self { tx }
    }

    pub fn add_message(&self, message: impl Into<String>) {
        tell!(self.tx, ExperimentReporterMsg::AddMessage { message: message.into() });
    }

    pub fn add_process_stdout(&self, line: impl Into<String>) {
        tell!(self.tx, ExperimentReporterMsg::AddProcessStdout { line: line.into() });
    }

    pub fn add_process_stderr(&self, line: impl Into<String>) {
        tell!(self.tx, ExperimentReporterMsg::AddProcessStderr { line: line.into() });
    }

    pub fn add_experiment_started(&self) {
        tell!(self.tx, ExperimentReporterMsg::AddExperimentStarted);
    }

    pub fn add_experiment_completed(&self) {
        tell!(self.tx, ExperimentReporterMsg::AddExperimentCompleted);
    }

    pub fn add_scenario_started(&self) {
        tell!(self.tx, ExperimentReporterMsg::AddScenarioStarted);
    }

    pub fn add_scenario_completed(&self, success: bool) {
        tell!(self.tx, ExperimentReporterMsg::AddScenarioCompleted { success });
    }
}