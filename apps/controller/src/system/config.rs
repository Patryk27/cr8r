use serde::Deserialize;

use crate::system::{AttachmentsConfig, EcosystemConfig};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemConfig {
    pub attachments: AttachmentsConfig,
    pub ecosystem: EcosystemConfig,
}