use serde::Deserialize;

use lib_sandbox::SandboxConfig;

use crate::system::AttachmentStoreConfig;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemConfig {
    pub runner: RunnerConfig,
    pub attachments: AttachmentStoreConfig,
    pub sandbox: SandboxConfig,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerConfig {
    pub name: String,
}
