use anyhow::*;
use lib_interop::models::DAttachmentId;
use log::*;
use std::collections::HashMap;

use crate::system::Attachment;

use super::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn download_attachments(&mut self) -> Result<HashMap<DAttachmentId, Attachment>> {
        debug!("Fetching experiment's attachments");

        let experiment_attachments = self.session
            .conn()
            .attachments()
            .find_many(self.experiment_id)
            .await?;

        debug!("About to download {} attachments", experiment_attachments.len());

        let mut attachments = HashMap::with_capacity(
            experiment_attachments.len(),
        );

        for experiment_attachment in experiment_attachments {
            let attachment = self.attachment_store
                .download(experiment_attachment.id)
                .await?;

            attachments.insert(experiment_attachment.id, attachment);
        }

        Ok(attachments)
    }
}