use std::fs;

use anyhow::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub controller: AppControllerConfig,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppControllerConfig {
    pub address: String,

    #[serde(default)]
    pub secret: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let file = fs::read_to_string("client.yaml")
            .context("Could not open file")?;

        let this = serde_yaml::from_str(&file)
            .context("Could not parse file as YAML")?;

        Ok(this)
    }
}