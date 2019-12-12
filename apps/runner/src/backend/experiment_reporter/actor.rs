use futures_util::StreamExt;
use log::*;

use lib_protocol::core::p_report::*;

use crate::backend::experiment_reporter::{ExperimentReporterMsg, ExperimentReporterRx};
use crate::core::ExperimentClient;

pub struct ExecutorReporterActor {
    rx: ExperimentReporterRx,
    client: ExperimentClient,
}

impl ExecutorReporterActor {
    pub fn new(rx: ExperimentReporterRx, client: ExperimentClient) -> Self {
        Self { rx, client }
    }

    pub async fn main(mut self) {
        debug!("Actor started");

        while let Some(msg) = self.rx.next().await {
            debug!("Processing message: {:?}", msg);

            let report = Self::msg_to_report(msg);

            if let Err(err) = self.client.report(report).await {
                error!("Could not send report to the controller: {:?}", err);
                // @todo try again in a moment
            }
        }

        debug!("Actor orphaned, halting");
    }

    fn msg_to_report(msg: ExperimentReporterMsg) -> Op {
        match msg {
            ExperimentReporterMsg::AddMessage { message } => {
                Op::Message(PMessage { message })
            }

            ExperimentReporterMsg::AddProcessStdout { line } => {
                Op::ProcessStdout(PProcessStdout { line })
            }

            ExperimentReporterMsg::AddProcessStderr { line } => {
                Op::ProcessStderr(PProcessStderr { line })
            }

            ExperimentReporterMsg::AddExperimentStarted => {
                Op::ExperimentStarted(PExperimentStarted {})
            }

            ExperimentReporterMsg::AddExperimentCompleted => {
                Op::ExperimentCompleted(PExperimentCompleted {})
            }

            ExperimentReporterMsg::AddScenarioStarted => {
                Op::ScenarioStarted(PScenarioStarted {})
            }

            ExperimentReporterMsg::AddScenarioCompleted { success } => {
                Op::ScenarioCompleted(PScenarioCompleted { success })
            }
        }
    }
}