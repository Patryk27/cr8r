use std::collections::HashSet;

use log::*;
use tokio::stream::StreamExt;
use tonic::transport::Channel;

use lib_core_channel::URx;
use lib_interop::domain::DAttachmentId;
use lib_interop::proto::services::attachments_client::AttachmentsClient;

use crate::system::AttachmentStore;

use super::{AttachmentStoreConfig, AttachmentStoreMsg};

pub struct AttachmentStoreActor {
    pub config: AttachmentStoreConfig,
    pub client: AttachmentsClient<Channel>,
    pub store: AttachmentStore,
    pub attachments: HashSet<DAttachmentId>,
}

impl AttachmentStoreActor {
    pub async fn start(mut self, mut mailbox: URx<AttachmentStoreMsg>) {
        trace!("Actor started");

        while let Some(msg) = mailbox.next().await {
            msg.handle(&mut self)
                .await;
        }

        // @todo remove attachments from filesystem

        trace!("Actor halted");
    }
}