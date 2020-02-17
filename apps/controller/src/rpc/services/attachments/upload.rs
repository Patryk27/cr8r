use anyhow::*;
use tokio::task::spawn;
use tonic::Streaming;

use lib_interop::proto::services::{PUploadAttachmentReply, PUploadAttachmentRequest};
use lib_interop::proto::services::p_upload_attachment_request::*;

use crate::system::AttachmentStore;

pub async fn upload_attachment(
    attachment_store: &AttachmentStore,
    mut request: Streaming<PUploadAttachmentRequest>,
) -> Result<PUploadAttachmentReply> {
    let metadata = request
        .message()
        .await?
        .map(|request| request.chunk)
        .flatten()
        .map(|chunk| {
            match chunk {
                Chunk::Metadata(metadata) => Some(metadata),
                _ => None,
            }
        })
        .flatten();

    let metadata = metadata.ok_or_else(|| {
        anyhow!("Protocol error: First chunk was expected to carry attachment's metadata")
    })?;

    let id = attachment_store
        .create(metadata.name.into(), metadata.size)
        .await?;

    let uploading_result = try {
        let attachment = attachment_store
            .find_one(id)
            .await?;

        while let Some(request) = request.message().await? {
            let body = request
                .chunk
                .map(|chunk| {
                    match chunk {
                        Chunk::Body(body) => Some(body),
                        _ => None,
                    }
                })
                .flatten();

            let content = body.ok_or_else(|| {
                anyhow!("Protocol error: Next chunk was expected to carry attachment's body")
            })?;

            attachment
                .add_chunk(content.body)
                .await?;
        }

        attachment
            .commit()
            .await?;
    };

    match uploading_result {
        Ok(()) => {
            Ok(PUploadAttachmentReply {
                id: id.into(),
            })
        }

        Err(err) => {
            let attachments = attachment_store.to_owned();

            spawn(async move {
                let _ = attachments
                    .remove(id)
                    .await;
            });

            Err(err)
        }
    }
}