use anyhow::*;
use tokio::{sync::mpsc, task};

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::domain::{DAttachmentId, DAttachmentName};
use lib_interop::proto::core::PAttachmentSize;

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
mod config;
mod error;
mod msg;

#[derive(Clone)]
pub struct Attachments {
    tx: UTx<AttachmentsMsg>,
}

impl Attachments {
    pub fn new(config: AttachmentsConfig) -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();

        ensure!(config.store_path.exists(), AttachmentsError::StoreNotExists {
            path: config.store_path,
        });

        task::spawn(AttachmentsActor {
            remaining_size: config.store_size,
            attachments: Default::default(),
            next_id: Default::default(),
            config,
        }.start(rx));

        Ok(Self { tx })
    }

    pub async fn create(&self, name: DAttachmentName, size: PAttachmentSize) -> Result<DAttachmentId> {
        ask!(self.tx, AttachmentsMsg::Create { name, size })
    }

    pub async fn get(&self, id: DAttachmentId) -> Result<Attachment> {
        ask!(self.tx, AttachmentsMsg::Get { id })
    }

    pub async fn remove(&self, id: DAttachmentId) -> Result<()> {
        ask!(self.tx, AttachmentsMsg::Remove { id })
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
                    .create("test.zip".into(), 1024)
                    .await
                    .unwrap();

                assert_eq!(DAttachmentId::from(1), id);
            }
        }

        mod given_too_big_attachment {
            use super::*;

            #[tokio::test]
            async fn returns_an_error() {
                let err = attachments()
                    .create("test.zip".into(), 1024 * 1024)
                    .await
                    .err()
                    .unwrap();

                assert_eq!(AttachmentsError::AttachmentTooLarge {
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
                    .create(name.clone(), 1024)
                    .await
                    .unwrap();

                let attachment = attachments
                    .get(id)
                    .await
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
                    .get(123.into())
                    .await
                    .err()
                    .unwrap();

                assert_eq!(AttachmentsError::AttachmentNotFound {
                    id: 123.into(),
                }.to_string(), err.to_string());
            }
        }
    }

    fn attachments() -> Attachments {
        Attachments::new(AttachmentsConfig {
            store_path: temp_dir(),
            store_size: 4096,
        }).unwrap()
    }
}