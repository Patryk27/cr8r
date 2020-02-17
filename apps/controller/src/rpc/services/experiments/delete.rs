use anyhow::*;

use lib_interop::proto::services::{PDeleteExperimentReply, PDeleteExperimentRequest};

use crate::system::ExperimentStore;

pub async fn delete_experiment(
    experiment_store: &ExperimentStore,
    request: PDeleteExperimentRequest,
) -> Result<PDeleteExperimentReply> {
    let id = request.id.into();

    experiment_store
        .delete(id)
        .await?;

    Ok(PDeleteExperimentReply {})
}