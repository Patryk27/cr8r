use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ExperimentReport {
    Message {
        message: String,
    },
}