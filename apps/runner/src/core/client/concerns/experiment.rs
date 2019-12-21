use chrono::Utc;

use lib_interop::protocol::core::{PExperimentEvent, PExperimentId, PRunnerId};
use lib_interop::protocol::core::p_experiment_event;

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

    pub async fn add_event(&mut self, report: p_experiment_event::Op) -> Result<()> {
        let event = PExperimentEvent {
            created_at: Utc::now().to_rfc3339(),
            op: Some(report),
        };

        self.client.add_experiment_event(
            self.runner.clone(),
            self.experiment.clone(),
            event,
        ).await?;

        Ok(())
    }
}