use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::domain::{DAttachmentId, DAttachmentName};
use lib_interop::proto::core::PAttachmentSize;

use super::AttachmentMsg;

pub struct AttachmentActor {
    pub id: DAttachmentId,
    pub name: DAttachmentName,
    pub size: PAttachmentSize,
}

impl AttachmentActor {
    pub async fn start(mut self, mut mailbox: URx<AttachmentMsg>) {
        trace!("Actor started");
        trace!("-> id = {}", self.id);
        trace!("-> name = {}", self.name);
        trace!("-> size = {}", self.size);

        while let Some(msg) = mailbox.next().await {
            msg.handle(&mut self);
        }

        trace!("Actor halted");
    }
}