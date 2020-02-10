use serde::Deserialize;

use lib_interop::proto::core::PAttachmentSize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct AttachmentsConfig {
    pub store_size: PAttachmentSize,
}