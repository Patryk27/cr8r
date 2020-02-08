use anyhow::*;
use tonic::IntoStreamingRequest;

use crate::client::ControllerClient;
use crate::proto::controller::*;

impl ControllerClient {
    pub async fn upload_attachment(
        &mut self,
        request: impl IntoStreamingRequest<Message=PUploadAttachmentRequest>,
    ) -> Result<PUploadAttachmentReply> {
        let reply = self.client
            .upload_attachment(request)
            .await?;

        Ok(reply.into_inner())
    }
}