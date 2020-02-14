use tonic::{Request, Response, Status, Streaming};

use lib_interop::proto::services::*;
use lib_interop::proto::services::attachments_server::Attachments;

use crate::system;

pub struct AttachmentsService {
    pub attachments: system::Attachments,
}

#[tonic::async_trait]
impl Attachments for AttachmentsService {
    type DownloadAttachmentStream = ();

    async fn download_attachment(
        &self,
        request: Request<PDownloadAttachmentRequest>,
    ) -> Result<Response<Self::DownloadAttachmentStream>, Status> {
        unimplemented!()
    }

    async fn upload_attachment(
        &self,
        request: Request<Streaming<PUploadAttachmentRequest>>,
    ) -> Result<Response<PUploadAttachmentReply>, Status> {
        unimplemented!()
    }
}