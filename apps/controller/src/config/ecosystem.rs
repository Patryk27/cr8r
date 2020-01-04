use std::collections::HashMap;

use serde::Deserialize;

pub use self::{
    environment::*,
    project::*,
    provider::*,
};

mod environment;
mod project;
mod provider;

pub type Providers = HashMap<String, Provider>;
pub type Projects = HashMap<String, Project>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ecosystem {
    #[serde(default)]
    pub environment: Environment,

    #[serde(default)]
    pub providers: Providers,

    pub projects: Projects,
}