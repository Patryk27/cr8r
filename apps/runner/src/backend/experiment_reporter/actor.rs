use futures_util::StreamExt;
use log::*;

use lib_protocol::core::report;

use crate::backend::{ExperimentReporterMsg, ExperimentReporterRx};
use crate::core::ExperimentClient;

pub struct ExecutorReporterActor {
    rx: ExperimentReporterRx,
    client: ExperimentClient,
}

impl ExecutorReporterActor {
    pub fn new(rx: ExperimentReporterRx, client: ExperimentClient) -> Self {
        Self { rx, client }
    }

    pub async fn start(mut self) {
        debug!("Actor started");

        while let Some(msg) = self.rx.next().await {
            debug!("Processing message: {:?}", msg);

            let report = Self::convert_msg_to_report(msg);

            if let Err(err) = self.client.report(report).await {
                error!("Could not send report to the controller: {:?}", err);
                // @todo try again in a moment
            }
        }

        debug!("Actor orphaned, halting");
    }

    fn convert_msg_to_report(msg: ExperimentReporterMsg) -> report::Op {
        match msg {
            ExperimentReporterMsg::ReportMessage { message } => {
                report::Op::Message(report::Message { message })
            }

            ExperimentReporterMsg::ReportProcessStdout { line } => {
                report::Op::ProcessStdout(report::ProcessStdout { line })
            }

            ExperimentReporterMsg::ReportProcessStderr { line } => {
                report::Op::ProcessStderr(report::ProcessStderr { line })
            }

            ExperimentReporterMsg::ReportExperimentStarted => {
                report::Op::ExperimentStarted(report::ExperimentStarted {})
            }

            ExperimentReporterMsg::ReportExperimentCompleted => {
                report::Op::ExperimentCompleted(report::ExperimentCompleted {})
            }

            ExperimentReporterMsg::ReportScenarioStarted => {
                report::Op::ScenarioStarted(report::ScenarioStarted {})
            }

            ExperimentReporterMsg::ReportScenarioCompleted { success } => {
                report::Op::ScenarioCompleted(report::ScenarioCompleted { success })
            }
        }
    }
}