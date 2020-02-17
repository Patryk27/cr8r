use serde::Deserialize;

use crate::system::{AttachmentStoreConfig, EcosystemConfig};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemConfig {
    pub attachments: AttachmentStoreConfig,
    pub ecosystem: EcosystemConfig,
}