use serde::Deserialize;

use crate::engines::{LxdConfig, ShellConfig};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum SandboxConfig {
    Lxd(LxdConfig),
    Shell(ShellConfig),
}
