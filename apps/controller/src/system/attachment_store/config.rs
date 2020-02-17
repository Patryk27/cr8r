use std::path::PathBuf;

use serde::Deserialize;

use lib_interop::proto::models::PAttachmentSize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct AttachmentStoreConfig {
    pub store_path: PathBuf,
    pub store_size: PAttachmentSize,
}