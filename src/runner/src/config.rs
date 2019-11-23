use std::fs;
use std::path::Path;

use serde::Deserialize;
use snafu::ResultExt;

use crate::error;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub runner: Runner,
    pub controller: Controller,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Runner {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Controller {
    pub secret: String,
    pub address: String,
}

pub fn from_file(file: &Path) -> Result<Config, error::Error> {
    let file = fs::read_to_string(file)
        .context(error::FailedToOpenConfig)?;

    serde_yaml::from_str(&file)
        .context(error::FailedToParseConfig)
}