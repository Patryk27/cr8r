use serde::{Deserialize, Serialize};

use crate::{ExperimentDefinition, ExperimentId, ExperimentStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct Experiment {
    pub id: ExperimentId,
    pub status: ExperimentStatus,
    pub definition: ExperimentDefinition,
}