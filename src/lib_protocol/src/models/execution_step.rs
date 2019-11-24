use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStep {
    Log {
        message: String,
    },

    RunCommand {
        command: String,
    },

    RunCommands {
        commands: Vec<String>,
    },

    Start,
}