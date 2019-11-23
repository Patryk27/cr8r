use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub enum ExperimentDefinition {
    #[structopt(name = "rust/try-toolchain")]
    RustTryToolchain {
        toolchain: String,
    },
}

