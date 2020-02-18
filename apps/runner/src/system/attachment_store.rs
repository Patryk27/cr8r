use anyhow::*;
use tokio::fs::metadata;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::clients::AttachmentClient;
use lib_interop::models::DAttachmentId;

use crate::system::Attachment;

use self::{
    actor::*,
    msg::*,
};
pub use self::{
    config::*,
    error::*,
};

mod actor;
mod error;
mod config;
mod msg;

#[derive(Clone)]
pub struct AttachmentStore {
    tx: UTx<AttachmentStoreMsg>,
}

impl AttachmentStore {
    pub async fn new(config: AttachmentStoreConfig, client: AttachmentClient) -> Result<Self> {
        let (tx, rx) = unbounded_channel();

        ensure!(metadata(&config.store_path).await.is_ok(), AttachmentStoreError::StoreNotFound {
            path: config.store_path,
        });

        spawn(AttachmentStoreActor {
            config,
            client,
            store: AttachmentStore { tx: tx.clone() },
            attachments: Default::default(),
        }.start(rx));

        Ok(Self { tx })
    }

    pub async fn download(&self, id: DAttachmentId) -> Result<Attachment> {
        ask!(&self.tx, AttachmentStoreMsg::Download { id })
    }

    pub fn release(&self, id: DAttachmentId) {
        tell!(&self.tx, AttachmentStoreMsg::Release { id });
    }
}