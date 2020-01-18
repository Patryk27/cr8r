use serde::Deserialize;

use lib_lxd::{LxdContainerName, LxdImageName};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LxdConfig {
    pub container: LxdContainerName,
    pub image: LxdImageName,
}