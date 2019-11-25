use lib_client_protocol::{CreateExperimentRequest, CreateExperimentResponse};
use lib_protocol_core::{Experiment, ExperimentDefinition, ExperimentId};

use crate::{Connector, Result};

impl Connector {
    pub fn launch_experiment(&self, experiment: ExperimentDefinition) -> Result<ExperimentId> {
        let request = CreateExperimentRequest {
            experiment,
        };

        let response: CreateExperimentResponse = self.post("/experiments")
            .json(&request)
            .send()?
            .json()?;

        Ok(response.experiment_id)
    }

    pub fn experiment(&self, id: String) -> Result<Experiment> {
        unimplemented!()
    }
}