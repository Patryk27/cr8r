use std::fs;

use anyhow::*;
use serde::Deserialize;

use crate::rpc::RpcConfig;
use crate::system::SystemConfig;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "controller")]
    pub rpc: RpcConfig,

    #[serde(flatten)]
    pub system: SystemConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let file = fs::read_to_string("controller.yaml")
            .context("Could not open file")?;

        let this = serde_yaml::from_str(&file)
            .context("Could not parse file as YAML")?;

        Ok(this)
    }
}