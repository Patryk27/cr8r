use serde::Deserialize;

use lib_compiler::Ecosystem;

use crate::system::AttachmentStoreConfig;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemConfig {
    pub attachments: AttachmentStoreConfig,
    pub ecosystem: Ecosystem,
}
