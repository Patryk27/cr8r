use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentDefinition {
    TryOs {
        os: String,
    },

    TryToolchain {
        toolchain: String,
    },
}

