use serde::{Deserialize, Serialize};

use crate::RunnerId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentStatus {
    AwaitingRunner,
    Finished,

    Running {
        runner_id: RunnerId,
    },
}