use anyhow::*;
use tonic::transport::Channel;

use lib_interop::conv;
use lib_interop::domain::attachment::DAttachment;
use lib_interop::domain::DExperimentId;
use lib_interop::proto::services::attachments_client::AttachmentsClient;
use lib_interop::proto::services::PFindAttachmentsRequest;

use crate::modules::app::AppContext;

pub struct AttachmentRepository {
    attachments_client: AttachmentsClient<Channel>,
}

impl AttachmentRepository {
    pub async fn new(ctxt: &mut AppContext) -> Result<Self> {
        Ok(Self {
            attachments_client: ctxt.attachments().await?,
        })
    }

    pub async fn find(&mut self, experiment_id: DExperimentId) -> Result<Vec<DAttachment>> {
        let attachments = self.attachments_client
            .find_attachments(PFindAttachmentsRequest { experiment_id: experiment_id.into() })
            .await?
            .into_inner()
            .attachments;

        Ok(conv!(attachments as [_?]))
    }
}