use std::path::PathBuf;

use log::*;
use tokio::fs::{File, remove_file};
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::domain::{DAttachmentId, DAttachmentName};
use lib_interop::proto::core::PAttachmentSize;

use super::{AttachmentMsg, AttachmentState};

pub struct AttachmentActor {
    pub id: DAttachmentId,
    pub name: DAttachmentName,
    pub size: PAttachmentSize,
    pub state: AttachmentState,
}

impl AttachmentActor {
    pub async fn start(mut self, path: PathBuf, file: File, mut mailbox: URx<AttachmentMsg>) {
        trace!("Actor started");
        trace!("-> id = {}", self.id);
        trace!("-> name = {}", self.name);
        trace!("-> size = {}", self.size);
        trace!("-> path = {}", path.display());

        self.state = AttachmentState::Pending {
            file,
            uploaded_bytes: 0,
        };

        while let Some(msg) = mailbox.next().await {
            if msg.handle(&mut self).await.actor_should_stop() {
                break;
            }
        }

        trace!("Actor is halting");

        if let Err(err) = remove_file(path).await {
            error!("Could not remove file for attachment [id={}]: {:?}", self.id, err);
        }

        trace!("Actor halted");
    }
}