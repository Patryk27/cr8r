use std::fs;
use std::path::Path;

use serde::Deserialize;
use snafu::ResultExt;

use crate::{error, Result};

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

pub fn from_file(file: &Path) -> Result<Config> {
    let file = fs::read_to_string(file)
        .context(error::FailedToOpenConfig)?;

    serde_yaml::from_str(&file)
        .context(error::FailedToParseConfig)
}