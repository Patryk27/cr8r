use anyhow::*;
use tokio::fs::metadata;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::models::{DAttachmentId, DAttachmentName};
use lib_interop::proto::models::PAttachmentSize;

use crate::system::{Attachment, SystemEventBus};

use self::{
    actor::*,
    msg::*,
};
pub use self::{
    config::*,
    error::*,
};

mod actor;
mod config;
mod error;
mod msg;

#[derive(Clone)]
pub struct AttachmentStore {
    tx: UTx<AttachmentStoreMsg>,
}

impl AttachmentStore {
    pub async fn new(bus: SystemEventBus, config: AttachmentStoreConfig) -> Result<Self> {
        let (tx, rx) = unbounded_channel();

        ensure!(metadata(&config.store_path).await.is_ok(), AttachmentStoreError::StoreNotFound {
            path: config.store_path,
        });

        spawn(AttachmentStoreActor {
            bus,
            remaining_size: config.store_size,
            attachments: Default::default(),
            next_id: Default::default(),
            config,
        }.start(rx));

        Ok(Self { tx })
    }

    pub async fn create(&self, name: DAttachmentName, size: PAttachmentSize) -> Result<DAttachmentId> {
        ask!(self.tx, AttachmentStoreMsg::Create { name, size })
    }

    pub async fn find_one(&self, id: DAttachmentId) -> Result<Attachment> {
        ask!(self.tx, AttachmentStoreMsg::FindOne { id })
    }

    pub async fn remove(&self, id: DAttachmentId) -> Result<()> {
        ask!(self.tx, AttachmentStoreMsg::Remove { id })
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use super::*;

    mod create {
        use super::*;

        mod given_correct_attachment {
            use super::*;

            #[tokio::test]
            async fn creates_an_attachment_and_returns_its_id() {
                let id = attachments()
                    .create("test.zip".into(), 1024).await
                    .unwrap();

                assert_eq!(DAttachmentId::from(1), id);
            }
        }

        mod given_too_big_attachment {
            use super::*;

            #[tokio::test]
            async fn returns_an_error() {
                let err = attachments()
                    .create("test.zip".into(), 1024 * 1024).await
                    .err()
                    .unwrap();

                assert_eq!(AttachmentStoreError::AttachmentTooLarge {
                    attachment_size: 1024 * 1024,
                    remaining_store_size: 4096,
                }.to_string(), err.to_string());
            }
        }
    }

    mod get {
        use super::*;

        mod given_an_existing_id {
            use super::*;

            #[tokio::test]
            async fn returns_that_attachment() {
                let attachments = attachments();

                let name = DAttachmentName::from("test.zip");

                let id = attachments
                    .create(name.clone(), 1024).await
                    .unwrap();

                let attachment = attachments
                    .find_one(id).await
                    .unwrap();

                assert_eq!(
                    name,
                    attachment.get_name().await,
                );
            }
        }

        mod given_a_non_existing_id {
            use super::*;

            #[tokio::test]
            async fn returns_error() {
                let err = attachments()
                    .find_one(123.into()).await
                    .err()
                    .unwrap();

                assert_eq!(AttachmentStoreError::AttachmentNotFound {
                    id: 123.into(),
                }.to_string(), err.to_string());
            }
        }
    }

    fn attachments() -> AttachmentStore {
        unimplemented!()
    }
}