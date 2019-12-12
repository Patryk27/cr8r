use chrono::Utc;

use lib_protocol::core::{PExperimentId, PReport, PRunnerId};
use lib_protocol::core::p_report;

use crate::core::{Client, Result};

#[derive(Clone)]
pub struct ExperimentClient {
    client: Client,
    runner: PRunnerId,
    experiment: PExperimentId,
}

impl ExperimentClient {
    pub fn new(client: Client, runner: PRunnerId, experiment: PExperimentId) -> Self {
        Self { client, runner, experiment }
    }

    pub async fn report(&mut self, report: p_report::Op) -> Result<()> {
        let report = PReport {
            created_at: Utc::now().to_rfc3339(),
            op: Some(report),
        };

        self.client.add_experiment_report(
            self.runner.clone(),
            self.experiment.clone(),
            report,
        ).await?;

        Ok(())
    }
}