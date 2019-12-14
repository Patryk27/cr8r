use std::fs;
use std::path::Path;

use serde::Deserialize;
use snafu::ResultExt;

use crate::{error, Result, StdResult};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub controller: Controller,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Controller {
    pub secret: String,
    pub address: String,
}

pub fn load(file: &Path) -> Result<Config> {
    (try {
        let config = fs::read_to_string(file)?;
        let config = serde_yaml::from_str(&config)?;

        config
    }: StdResult<Config>).context(error::CouldntStart)
}