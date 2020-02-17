use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct AttachmentStoreConfig {
    pub store_path: PathBuf,
}
