use core::result;

use anyhow::*;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::mpsc::channel;
use tokio::task::spawn;
use tonic::Status;

use lib_interop::proto::services::{PDownloadAttachmentReply, PDownloadAttachmentRequest};
use lib_interop::proto::services::p_download_attachment_reply::{Chunk, PBody};

use crate::system::AttachmentStore;

use super::transform_error;

const CHUNK_SIZE: usize = 64 * 1024;

pub async fn download_attachment(
    attachment_store: &AttachmentStore,
    request: PDownloadAttachmentRequest,
) -> Result<impl Stream<Item=result::Result<PDownloadAttachmentReply, Status>>> {
    let id = request.id.into();

    let download_token = attachment_store
        .find_one(id).await?
        .download().await?;

    let (mut tx, rx) = channel(32);

    spawn(async move {
        let result = try {
            let mut file = BufReader::new(
                File::open(download_token.path()).await?
            );

            let mut chunk = [0u8; CHUNK_SIZE];

            loop {
                let read_bytes = file.read(&mut chunk).await?;

                if read_bytes == 0 {
                    break;
                }

                let mut chunk = chunk.to_vec();

                chunk.truncate(read_bytes);

                tx.send(Ok(PDownloadAttachmentReply {
                    chunk: Some(Chunk::Body(PBody {
                        body: chunk,
                    }))
                })).await?;
            }
        };

        if let Err(err) = result {
            let _ = tx.send(Err(err)).await;
        }

        // We have to keep the token alive for the entire time we're reading the attachment - so, just to be sure,
        // we're dropping it manually at the end
        drop(download_token);
    });

    let rx = rx.map(|chunk| {
        chunk.map_err(transform_error)
    });

    Ok(rx)
}
