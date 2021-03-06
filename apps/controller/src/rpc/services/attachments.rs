use tokio::stream::Stream;
use tonic::{Request, Response, Status, Streaming};

use lib_interop::proto::services::*;
use lib_interop::proto::services::attachments_server::Attachments;

use crate::system::{AttachmentStore, ExperimentStore};

use super::transform_error;

mod download;
mod find;
mod upload;

pub struct AttachmentsService {
    pub attachment_store: AttachmentStore,
    pub experiment_store: ExperimentStore,
}

#[tonic::async_trait]
impl Attachments for AttachmentsService {
    type DownloadAttachmentStream = impl Stream<Item=Result<PDownloadAttachmentReply, Status>>;

    async fn download_attachment(
        &self,
        request: Request<PDownloadAttachmentRequest>,
    ) -> Result<Response<Self::DownloadAttachmentStream>, Status> {
        download::download_attachment(&self.attachment_store, request.into_inner()).await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn find_attachments(
        &self,
        request: Request<PFindAttachmentsRequest>,
    ) -> Result<Response<PFindAttachmentsReply>, Status> {
        find::find(&self.experiment_store, request.into_inner()).await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn upload_attachment(
        &self,
        request: Request<Streaming<PUploadAttachmentRequest>>,
    ) -> Result<Response<PUploadAttachmentReply>, Status> {
        upload::upload_attachment(&self.attachment_store, request.into_inner()).await
            .map(Response::new)
            .map_err(transform_error)
    }
}