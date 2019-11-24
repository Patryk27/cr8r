use serde::{Deserialize, Serialize};

use crate::{ExecutionPlan, ExperimentDefinition, ExperimentId, ExperimentStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: ExperimentId,
    pub status: ExperimentStatus,
    pub definition: ExperimentDefinition,
    pub execution_plans: Vec<ExecutionPlan>,
}