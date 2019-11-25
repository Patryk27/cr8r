use serde::{Deserialize, Serialize};
use uuid::Uuid;

use lib_protocol_core::ExperimentDefinition;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExperimentRequest {
    pub experiment: ExperimentDefinition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExperimentResponse {
    pub experiment_id: Uuid,
}