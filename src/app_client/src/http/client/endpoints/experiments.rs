use lib_protocol::{ExperimentDefinition, ExperimentId};
use lib_protocol::targets::client::{CreateExperimentRequest, CreateExperimentResponse};

use crate::{Client, Result};

impl Client {
    pub fn create_experiment(&mut self, experiment: ExperimentDefinition) -> Result<ExperimentId> {
        let request = CreateExperimentRequest {
            experiment,
        };

        let response: CreateExperimentResponse = self.post("/experiments")
            .json(&request)
            .send()?
            .json()?;

        Ok(response.experiment_id)
    }
}