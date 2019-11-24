use serde::{Deserialize, Serialize};

use crate::ExecutionStep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub project_name: String,
    pub project_repository: String,
    pub os: String,
    pub toolchain: String,
    pub steps: Vec<ExecutionStep>,
}