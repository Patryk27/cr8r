use std::path::PathBuf;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShellConfig {
    pub root: PathBuf,
}