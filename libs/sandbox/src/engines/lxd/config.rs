use std::path::PathBuf;

use serde::Deserialize;

use lib_lxd::{LxdContainerName, LxdImageName};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LxdSandboxConfig {
    pub container: LxdContainerName,
    pub image: LxdImageName,

    #[serde(default = "defaults::root")]
    pub root: PathBuf,

    #[serde(default)]
    pub before_boot: Option<Vec<String>>,

    #[serde(default)]
    pub after_boot: Option<Vec<String>>,

    #[serde(default = "defaults::forward_ssh")]
    pub forward_ssh: bool,

    #[serde(default = "defaults::wait_for_network")]
    pub wait_for_network: bool,

    #[serde(default = "defaults::install_rustup")]
    pub install_rustup: bool,
}

mod defaults {
    use super::*;

    pub fn root() -> PathBuf {
        "/root".into()
    }

    pub fn forward_ssh() -> bool {
        false
    }

    pub fn wait_for_network() -> bool {
        true
    }

    pub fn install_rustup() -> bool {
        true
    }
}