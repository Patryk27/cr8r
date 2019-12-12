use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;
use log::*;
use serde::Deserialize;
use snafu::ResultExt;

use crate::{error, Result, StdResult};

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

pub fn load() -> Result<Config> {
    let file = PathBuf::from("runner.yaml");

    from_file(&file)
}

pub fn from_file(file: &Path) -> Result<Config> {
    info!("Loading configuration from: {}", file.display().to_string().green());

    (try {
        let config = fs::read_to_string(file)?;
        let config = serde_yaml::from_str(&config)?;

        config
    }: StdResult<Config>).context(error::FailedToStart)
}