use anyhow::Result;

use lib_interop::client::ControllerClient;
use lib_interop::proto::core::{PEvent, PExperimentId, PRunnerId};

#[derive(Clone)]
pub struct ExperimentClient {
    client: ControllerClient,
    runner: PRunnerId,
    experiment: PExperimentId,
}

impl ExperimentClient {
    pub fn new(client: ControllerClient, runner: PRunnerId, experiment: PExperimentId) -> Self {
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