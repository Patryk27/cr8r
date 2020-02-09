use anyhow::*;

use lib_interop::domain::DAttachmentId;
// @todo use `DAttachmentName` and `DAttachmentSize`
use lib_interop::proto::core::{PAttachmentName, PAttachmentSize};

use crate::system::Attachment;

#[derive(Clone)]
pub struct Attachments {
    //
}

impl Attachments {
    pub fn new() -> Self {
        Self {
            //
        }
    }

    pub async fn create(&self, name: PAttachmentName, size: PAttachmentSize) -> Result<DAttachmentId> {
        unimplemented!()
    }

    pub async fn get(&self, id: DAttachmentId) -> Result<Attachment> {
        unimplemented!()
    }
}