use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScenarioStep {
    Log {
        message: String,
    },

    Command {
        command: String,
    },

    Start,
}