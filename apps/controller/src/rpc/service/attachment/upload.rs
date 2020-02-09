use anyhow::*;
use tonic::Streaming;

use lib_interop::proto::controller::*;
use lib_interop::proto::controller::p_upload_attachment_request::*;

pub async fn upload_attachment(mut request: Streaming<PUploadAttachmentRequest>) -> Result<PUploadAttachmentReply> {
    let mut total = 0;

    while let Some(msg) = request.message().await? {
        if let Some(chunk) = msg.chunk {
            match chunk {
                Chunk::Metadata(metadata) => {
                    println!("meta: {:#?}", metadata);
                }

                Chunk::Content(content) => {
                    total += content.size;

                    tokio::time::delay_for(tokio::time::Duration::from_millis(10))
                        .await;
                }
            }
        }
    }

    println!("tots: {} bytes", total);

    Ok(PUploadAttachmentReply {
        id: 1, // @todo
    })
}