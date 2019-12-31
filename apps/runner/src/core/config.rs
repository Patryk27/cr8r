use std::fs;
use std::path::Path;

use serde::Deserialize;
use snafu::ResultExt;

use crate::{error, Result, StdResult};

pub use self::{
    controller::*,
    runner::*,
    sandbox::*,
};

mod controller;
mod runner;
mod sandbox;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub runner: RunnerConfig,
    pub controller: ControllerConfig,
    pub sandbox: SandboxConfig,
}

pub fn load(file: &Path) -> Result<Config> {
    (try {
        let file = fs::read_to_string(file)?;
        serde_yaml::from_str(&file)?
    }: StdResult<Config>).context(error::CouldntStart)
}