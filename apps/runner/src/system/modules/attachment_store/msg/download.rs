use std::fs::File as StdFile;
use std::io::BufReader as StdBufReader;

use anyhow::*;
use log::*;
use tar::Archive as TarArchive;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use tokio::stream::StreamExt;
use tokio::task::spawn_blocking;

use lib_interop::models::DAttachmentId;
use lib_interop::proto::services::p_download_attachment_reply::{Chunk, PBody};

use crate::system::Attachment;

use super::super::{AttachmentStoreActor, AttachmentStoreError};

// @todo if downloading attachment fails, we can early-delete it; a.k.a.: put a `try` block inside
pub async fn download(actor: &mut AttachmentStoreActor, id: DAttachmentId) -> Result<Attachment> {
    ensure!(!actor.attachments.contains(&id), AttachmentStoreError::AttachmentSlotTaken { id });

    let att_archive = actor.attachment_archive(id);
    let att_dir = actor.attachment_dir(id);

    {
        info!("Downloading attachment: id={}, path={}", id, att_dir.display());

        let mut file = TokioFile::create(&att_archive).await?;
        let mut chunks = actor.client.download(id).await?;

        while let Some(chunk) = chunks.next().await {
            match chunk? {
                Chunk::Body(PBody { body }) => {
                    file.write(&body).await?;
                }
            }
        }

        file.sync_all().await?;
    }

    {
        info!("Extracting attachment: id={}", id);

        let att_dir = att_dir.clone();

        let result: Result<()> = spawn_blocking(move || {
            let buf = StdBufReader::new(
                StdFile::open(att_archive)?
            );

            let mut archive = TarArchive::new(buf);

            archive.unpack(att_dir)?;

            Ok(())
        }).await?;

        result?;
    }

    actor.attachments.insert(id);

    Ok(Attachment::new(
        id,
        att_dir,
        actor.store.clone(),
    ))
}
