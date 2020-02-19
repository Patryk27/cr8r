use std::collections::HashMap;

use serde::Deserialize;

use crate::system::AttachmentStoreConfig;

pub use self::ecosystem::*;

mod ecosystem;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemConfig {
    pub attachments: AttachmentStoreConfig,
    pub ecosystem: EcosystemConfig,
}

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
