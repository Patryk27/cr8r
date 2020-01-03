use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub controller: ControllerConfig,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControllerConfig {
    pub address: String,

    #[serde(default)]
    pub secret: Option<String>,
}

pub fn load() -> Result<Config> {
    let file = fs::read_to_string(
        &PathBuf::from("client.yaml")
    )?;

    let config = serde_yaml::from_str(&file)?;

    Ok(config)
}