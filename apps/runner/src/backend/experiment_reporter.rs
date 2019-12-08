use futures_channel::mpsc;

use lib_actor::tell;

use crate::core::ExperimentClient;

pub use self::{
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
        ).start());

        Self { tx }
    }

    pub fn report_message(&self, message: impl Into<String>) {
        tell!(self.tx, ExperimentReporterMsg::ReportMessage { message: message.into() });
    }

    pub fn report_process_stdout(&self, line: impl Into<String>) {
        tell!(self.tx, ExperimentReporterMsg::ReportProcessStdout { line: line.into() });
    }

    pub fn report_process_stderr(&self, line: impl Into<String>) {
        tell!(self.tx, ExperimentReporterMsg::ReportProcessStderr { line: line.into() });
    }

    pub fn report_experiment_started(&self) {
        tell!(self.tx, ExperimentReporterMsg::ReportExperimentStarted);
    }

    pub fn report_experiment_completed(&self) {
        tell!(self.tx, ExperimentReporterMsg::ReportExperimentCompleted);
    }

    pub fn report_scenario_started(&self) {
        tell!(self.tx, ExperimentReporterMsg::ReportScenarioStarted);
    }

    pub fn report_scenario_completed(&self, success: bool) {
        tell!(self.tx, ExperimentReporterMsg::ReportScenarioCompleted { success });
    }
}