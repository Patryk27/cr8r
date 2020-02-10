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

pub type EcosystemProvidersConfig = HashMap<String, EcosystemProviderConfig>;
pub type EcosystemProjectsConfig = HashMap<String, EcosystemProjectConfig>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcosystemConfig {
    #[serde(default)]
    pub environment: EcosystemEnvironmentConfig,

    #[serde(default)]
    pub providers: EcosystemProvidersConfig,

    pub projects: EcosystemProjectsConfig,
}
