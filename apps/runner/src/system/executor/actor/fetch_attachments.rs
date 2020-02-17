use std::collections::HashMap;

use anyhow::*;
use log::*;

use lib_interop::domain::DAttachmentId;
use lib_interop::proto::services::PFindAttachmentsRequest;

use crate::system::Attachment;

use super::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn fetch_attachments(&mut self) -> Result<HashMap<DAttachmentId, Attachment>> {
        debug!("Fetching experiment's attachments");

        let pending_attachments = self.session.conn
            .attachments()
            .find_attachments(PFindAttachmentsRequest { experiment_id: self.experiment_id.into() })
            .await?
            .into_inner()
            .attachments;

        debug!("About to download {} attachments", pending_attachments.len());

        let mut attachments = HashMap::with_capacity(
            pending_attachments.len(),
        );

        for attachment in pending_attachments {
            let attachment_id = attachment.id.into();

            let attachment = self.attachment_store
                .download(attachment_id)
                .await?;

            attachments.insert(attachment_id, attachment);
        }

        Ok(attachments)
    }
}