use serde::{Deserialize, Serialize};

use crate::{ExperimentDefinition, ExperimentId, ExperimentStatus, Scenario};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: ExperimentId,
    pub status: ExperimentStatus,
    pub definition: ExperimentDefinition,
    pub scenarios: Vec<Scenario>,
}