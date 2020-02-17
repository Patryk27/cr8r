use std::convert::TryInto;

use anyhow::*;

use lib_interop::proto::services::{PCreateExperimentReply, PCreateExperimentRequest};

use crate::system::ExperimentStore;

pub async fn create_experiment(
    experiment_store: &ExperimentStore,
    request: PCreateExperimentRequest,
) -> Result<PCreateExperimentReply> {
    let definition = request.definition
        .ok_or_else(|| anyhow!("No experiment definition has been provided"))?
        .try_into()?;

    let id = experiment_store
        .launch(definition)
        .await?;

    Ok(PCreateExperimentReply {
        id: id.into(),
    })
}