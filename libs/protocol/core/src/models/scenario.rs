use serde::{Deserialize, Serialize};

use crate::ScenarioStep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub project_name: String,
    pub project_repository: String,
    pub os: String,
    pub toolchain: String,
    pub steps: Vec<ScenarioStep>,
}