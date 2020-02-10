use anyhow::*;
use tonic::Streaming;

use lib_interop::proto::controller::*;
use lib_interop::proto::controller::p_upload_attachment_request::*;

use crate::system::System;

pub async fn upload_attachment(
    system: &System,
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

    let id = system
        .attachments
        .create(metadata.name.into(), metadata.size.into())
        .await?;

    let attachment = system
        .attachments
        .get(id)
        .await?;

    while let Some(request) = request.message().await? {
        let content = request
            .chunk
            .map(|chunk| {
                match chunk {
                    Chunk::Content(content) => Some(content),
                    _ => None,
                }
            })
            .flatten();

        let content = content.ok_or_else(|| {
            anyhow!("Protocol error: Next chunk was expected to carry attachment's contents")
        })?;

        // attachment.upload_chunk() @todo
    }

    Ok(PUploadAttachmentReply {
        id: id.into(),
    })
}