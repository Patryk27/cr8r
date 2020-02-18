use anyhow::*;
use log::*;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::stream::StreamExt;

use lib_interop::models::DAttachmentId;
use lib_interop::proto::services::p_download_attachment_reply::{Chunk, PBody};

use crate::system::Attachment;

use super::super::{AttachmentStoreActor, AttachmentStoreError};

pub async fn download(actor: &mut AttachmentStoreActor, id: DAttachmentId) -> Result<Attachment> {
    ensure!(!actor.attachments.contains(&id), AttachmentStoreError::AttachmentSlotTaken { id });

    let path = actor.config
        .store_path
        .join(format!("{}", id));

    {
        info!("Downloading attachment: id={}, path={}", id, path.display());

        let mut file = File::create(path)
            .await?;

        let mut chunks = actor.client
            .download(id)
            .await?;

        while let Some(chunk) = chunks.next().await {
            match chunk {
                Chunk::Body(PBody { body }) => {
                    file.write(&body)
                        .await?;
                }
            }
        }

        file.sync_all()
            .await?;
    }

    {
        info!("Extracting attachment: id={}", id);

        // @todo
    }

    actor.attachments.insert(id);

    Ok(Attachment::new(
        id,
        Default::default(),
        actor.store.clone(),
    ))
}