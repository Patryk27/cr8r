use anyhow::*;

use lib_interop::proto::services::{PFindAttachmentsReply, PFindAttachmentsRequest};

use crate::system::ExperimentStore;

pub async fn find(
    experiment_store: &ExperimentStore,
    request: PFindAttachmentsRequest,
) -> Result<PFindAttachmentsReply> {
    let experiment_id = request.experiment_id.into();

    let experiment = experiment_store
        .find_one(experiment_id)
        .await?;

    let mut attachments = Vec::new();

    for attachment in experiment.get_attachments().await {
        let attachment = attachment
            .get_model()
            .await;

        attachments.push(
            attachment.into(),
        );
    }

    Ok(PFindAttachmentsReply {
        attachments,
    })
}