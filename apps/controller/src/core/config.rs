use std::fs;
use std::path::Path;

use serde::Deserialize;
use snafu::ResultExt;

use crate::core::{error, Result, StdResult};

pub use self::{
    controller::*,
    ecosystem::*,
};

mod controller;
mod ecosystem;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub controller: Controller,
    pub ecosystem: Ecosystem,
}

pub fn load(file: &Path) -> Result<Config> {
    (try {
        let config = fs::read_to_string(file)?;
        let config = serde_yaml::from_str(&config)?;

        config
    }: StdResult<Config>).context(error::FailedToConfigure)
}