use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub enum ExperimentDefinition {
    #[structopt(name = "try-os")]
    TryOs {
        os: String,
    },

    #[structopt(name = "try-toolchain")]
    TryToolchain {
        toolchain: String,
    },
}

