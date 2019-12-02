use chrono::Utc;

use lib_protocol::core::{ExperimentId, report, Report, RunnerId};

use crate::core::{Client, Result};

#[derive(Clone)]
pub struct ExperimentClient {
    client: Client,
    runner: RunnerId,
    experiment: ExperimentId,
}

impl ExperimentClient {
    pub fn new(client: Client, runner: RunnerId, experiment: ExperimentId) -> Self {
        Self { client, runner, experiment }
    }

    pub async fn report_ping(&mut self) -> Result<()> {
        self.report(report::Op::Ping(report::Ping {
            //
        })).await
    }

    pub async fn report_message(&mut self, message: impl Into<String>) -> Result<()> {
        self.report(report::Op::Message(report::Message {
            message: message.into(),
        })).await
    }

    pub async fn report_output(&mut self, output: impl Into<String>) -> Result<()> {
        self.report(report::Op::Output(report::Output {
            output: output.into(),
        })).await
    }

    pub async fn report_experiment_started(&mut self) -> Result<()> {
        self.report(report::Op::ExperimentStarted(report::ExperimentStarted {
            //
        })).await
    }

    pub async fn report_experiment_completed(&mut self) -> Result<()> {
        self.report(report::Op::ExperimentCompleted(report::ExperimentCompleted {
            //
        })).await
    }

    pub async fn report_scenario_started(&mut self) -> Result<()> {
        self.report(report::Op::ScenarioStarted(report::ScenarioStarted {
            // @todo pass scenario id / project name / anything
        })).await
    }

    pub async fn report_scenario_completed(&mut self, success: bool) -> Result<()> {
        self.report(report::Op::ScenarioCompleted(report::ScenarioCompleted {
            success,
        })).await
    }

    async fn report(&mut self, report: report::Op) -> Result<()> {
        let report = Report {
            created_at: Utc::now().to_rfc3339(),
            op: Some(report),
        };

        self.client.report_experiment(
            self.runner.clone(),
            self.experiment.clone(),
            report,
        ).await?;

        Ok(())
    }
}