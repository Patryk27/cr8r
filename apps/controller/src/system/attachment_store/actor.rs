use std::collections::HashMap;

use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::models::DAttachmentId;
use lib_interop::proto::models::PAttachmentSize;

use crate::system::Attachment;

use super::{AttachmentStoreConfig, AttachmentStoreMsg};

pub struct AttachmentStoreActor {
    pub config: AttachmentStoreConfig,
    pub remaining_size: PAttachmentSize,
    pub attachments: HashMap<DAttachmentId, Attachment>,
    pub next_id: DAttachmentId,
}

impl AttachmentStoreActor {
    pub async fn start(mut self, mut mailbox: URx<AttachmentStoreMsg>) {
        trace!("Actor started");

        while let Some(msg) = mailbox.next().await {
            msg.handle(&mut self)
                .await;
        }

        trace!("Actor halted");
    }
}