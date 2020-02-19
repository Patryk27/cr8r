use std::collections::HashSet;
use std::path::PathBuf;

use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::clients::AttachmentClient;
use lib_interop::models::DAttachmentId;

use crate::system::AttachmentStore;

use super::{AttachmentStoreConfig, AttachmentStoreMsg};

pub struct AttachmentStoreActor {
    pub config: AttachmentStoreConfig,
    pub client: AttachmentClient,
    pub store: AttachmentStore,
    pub attachments: HashSet<DAttachmentId>,
}

impl AttachmentStoreActor {
    pub async fn start(mut self, mut mailbox: URx<AttachmentStoreMsg>) {
        trace!("Actor started");

        while let Some(msg) = mailbox.next().await {
            msg.handle(&mut self).await;
        }

        // @todo remove attachments from filesystem

        trace!("Actor halted");
    }

    pub(super) fn attachment_archive(&self, id: DAttachmentId) -> PathBuf {
        self.config.store_path.with_file_name(format!("{}.archive", id))
    }

    pub(super) fn attachment_dir(&self, id: DAttachmentId) -> PathBuf {
        self.config.store_path.join(format!("{}", id))
    }
}