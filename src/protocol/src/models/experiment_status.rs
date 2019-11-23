use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentStatus {
    Awaiting,
    Finished,
    Running,
}