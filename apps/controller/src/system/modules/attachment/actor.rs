use std::path::PathBuf;
use std::sync::atomic::Ordering;

use chrono::{DateTime, Utc};
use log::*;
use tokio::fs::{File, remove_file};
use tokio::stream::StreamExt;
use tokio::time::{delay_for, Duration};

use lib_core_channel::URx;
use lib_interop::models::{DAttachmentId, DAttachmentName};
use lib_interop::proto::models::PAttachmentSize;

use super::{AttachmentMsg, AttachmentStatus};

pub struct AttachmentActor {
    pub id: DAttachmentId,
    pub name: DAttachmentName,
    pub size: PAttachmentSize,
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub status: AttachmentStatus,
}

impl AttachmentActor {
    pub async fn start(mut self, file: File, mut mailbox: URx<AttachmentMsg>) {
        trace!("Actor started");
        trace!("-> id = {}", self.id);
        trace!("-> name = {}", self.name);
        trace!("-> size = {}", self.size);
        trace!("-> path = {}", self.path.display());

        self.status = AttachmentStatus::Pending {
            file,
            uploaded_bytes: 0,
        };

        while let Some(msg) = mailbox.next().await {
            if msg.handle(&mut self).await.actor_should_stop() {
                break;
            }
        }

        trace!("Actor is halting");

        self.clean_up().await;

        trace!("Actor halted");
    }

    async fn clean_up(&mut self) {
        if let AttachmentStatus::Ready { active_download_tokens } = &self.status {
            // As long as someone holds a download token, we cannot remove attachment from the filesystem - so let's
            // just wait until all tokens are back.
            //
            // We could use some more sophisticated mechanism (e.g. a condvar), but polling is good enough.
            while active_download_tokens.load(Ordering::SeqCst) != 0 {
                delay_for(Duration::from_secs(1)).await;
            }
        }

        if let Err(err) = remove_file(&self.path).await {
            error!("Could not remove file for attachment [id={}]: {:?}", self.id, err);
        }
    }
}