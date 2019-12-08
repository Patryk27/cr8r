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

    pub async fn report(&mut self, report: report::Op) -> Result<()> {
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