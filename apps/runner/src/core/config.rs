use std::fs;

use anyhow::Result;
use serde::Deserialize;

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

impl Config {
    pub fn load() -> Result<Self> {
        let file = fs::read_to_string("runner.yaml")?;
        let this = serde_yaml::from_str(&file)?;

        Ok(this)
    }
}