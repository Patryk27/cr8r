use std::fs;

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

impl Config {
    pub fn load() -> Result<Self> {
        let file = fs::read_to_string("client.yaml")?;
        let this = serde_yaml::from_str(&file)?;

        Ok(this)
    }
}