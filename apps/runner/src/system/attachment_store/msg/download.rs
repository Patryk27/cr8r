use anyhow::*;
use log::*;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use lib_interop::domain::DAttachmentId;
use lib_interop::proto::services::p_download_attachment_reply::{Chunk, PBody};
use lib_interop::proto::services::PDownloadAttachmentRequest;

use crate::system::Attachment;

use super::super::{AttachmentStoreActor, AttachmentStoreError};

pub async fn download(actor: &mut AttachmentStoreActor, id: DAttachmentId) -> Result<Attachment> {
    ensure!(!actor.attachments.contains(&id), AttachmentStoreError::AttachmentSlotTaken { id });

    let path = actor.config
        .store_path
        .join(format!("{}", id));

    info!("Downloading attachment: id={}, path={}", id, path.display());

    let mut file = File::create(path)
        .await?;

    let mut stream = actor.client
        .download_attachment(PDownloadAttachmentRequest { id: id.into() })
        .await?
        .into_inner();

    while let Some(msg) = stream.message().await? {
        if let Some(chunk) = msg.chunk {
            match chunk {
                Chunk::Body(PBody { body }) => {
                    file.write(&body)
                        .await?;
                }
            }
        }
    }

    file.sync_all()
        .await?;

    info!("Attachment downloaded");

    actor.attachments.insert(id);

    Ok(Attachment::new(
        id,
        Default::default(),
        actor.store.clone(),
    ))
}