use chrono::Utc;

use lib_interop::protocol::core::{PEvent, PExperimentId, PRunnerId};

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

    pub async fn add_event(&mut self, event: PEvent) -> Result<()> {
        self.client.add_event(
            self.runner.clone(),
            self.experiment.clone(),
            event,
        ).await?;

        Ok(())
    }
}