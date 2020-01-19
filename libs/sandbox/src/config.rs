use serde::Deserialize;

use crate::engines::{LxdSandboxConfig, ShellSandboxConfig};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type", content = "config")]
pub enum SandboxConfig {
    Lxd(LxdSandboxConfig),
    Shell(ShellSandboxConfig),
}
