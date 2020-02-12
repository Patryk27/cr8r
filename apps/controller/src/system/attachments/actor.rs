use std::collections::HashMap;

use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::domain::DAttachmentId;
use lib_interop::proto::core::PAttachmentSize;

use crate::system::Attachment;

use super::{AttachmentsConfig, AttachmentsMsg};

pub struct AttachmentsActor {
    pub config: AttachmentsConfig,
    pub remaining_size: PAttachmentSize,
    pub attachments: HashMap<DAttachmentId, Attachment>,
    pub next_id: DAttachmentId,
}

impl AttachmentsActor {
    pub async fn start(mut self, mut mailbox: URx<AttachmentsMsg>) {
        trace!("Actor started");

        while let Some(msg) = mailbox.next().await {
            msg.handle(&mut self)
                .await;
        }

        trace!("Actor halted");
    }
}