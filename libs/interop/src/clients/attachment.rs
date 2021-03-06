use anyhow::*;
use tokio::stream::{Stream, StreamExt};
use tonic::transport::Channel;

use crate::connection::Connection;
use crate::conv;
use crate::models::{DAttachmentId, DExperimentId};
use crate::models::attachment::DAttachment;
use crate::proto::services::*;
use crate::proto::services::attachments_client::AttachmentsClient as AttachmentsClientInner;
use crate::proto::services::p_download_attachment_reply::Chunk as PDownloadChunk;
use crate::proto::services::p_upload_attachment_request::Chunk as PUploadChunk;

#[derive(Clone)]
pub struct AttachmentClient {
    inner: AttachmentsClientInner<Channel>,
}

impl AttachmentClient {
    crate fn new(conn: Connection) -> Self {
        Self {
            inner: AttachmentsClientInner::with_interceptor(
                conn.channel,
                conn.interceptor,
            ),
        }
    }

    pub async fn download(&mut self, id: DAttachmentId) -> Result<impl Stream<Item=Result<PDownloadChunk>>> {
        let chunks = self.inner
            .download_attachment(PDownloadAttachmentRequest { id: id.into() }).await?
            .into_inner();

        let chunks = chunks
            .map(|chunk| Ok(chunk?.chunk))
            .filter_map(Result::transpose);

        Ok(chunks)
    }

    pub async fn find_many(&mut self, experiment_id: DExperimentId) -> Result<Vec<DAttachment>> {
        let attachments = self.inner
            .find_attachments(PFindAttachmentsRequest { experiment_id: experiment_id.into() }).await?
            .into_inner()
            .attachments;

        Ok(conv!(attachments as [_?]))
    }

    pub async fn upload(&mut self, chunks: impl Stream<Item=PUploadChunk> + Send + Sync + 'static) -> Result<DAttachmentId> {
        let chunks = chunks.map(|chunk| {
            PUploadAttachmentRequest {
                chunk: Some(chunk),
            }
        });

        let reply = self.inner
            .upload_attachment(chunks).await?
            .into_inner();

        Ok(reply.id.into())
    }
}