use lib_protocol as proto;

use crate::{Client, Result};

impl Client {
    pub fn create_experiment(&mut self, experiment: proto::ExperimentDefinition) -> Result<proto::ExperimentId> {
        let request = proto::client::CreateExperimentRequest {
            experiment,
        };

        let response: proto::client::CreateExperimentResponse = self.post("/experiments")
            .json(&request)
            .send()?
            .json()?;

        Ok(response.experiment_id)
    }
}