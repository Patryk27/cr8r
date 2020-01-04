use std::fs;

use anyhow::Result;
use serde::Deserialize;

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

impl Config {
    pub fn load() -> Result<Self> {
        let file = fs::read_to_string("controller.yaml")?;
        let this = serde_yaml::from_str(&file)?;

        Ok(this)
    }
}