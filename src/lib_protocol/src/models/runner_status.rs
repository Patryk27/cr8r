use serde::{Deserialize, Serialize};

use crate::ExperimentId;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RunnerStatus {
    Idle,

    Initializing,

    Working {
        experiment_id: ExperimentId,
    },
}