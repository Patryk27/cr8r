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
    pub address: String,

    #[serde(default)]
    pub secret: Option<String>,
}

pub fn load(file: &Path) -> Result<Config> {
    (try {
        let file = fs::read_to_string(file)?;
        serde_yaml::from_str(&file)?
    }: StdResult<Config>).context(error::CouldntStart)
}