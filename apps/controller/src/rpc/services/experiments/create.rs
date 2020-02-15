use std::convert::TryInto;

use anyhow::*;

use lib_interop::proto::services::{PCreateExperimentReply, PCreateExperimentRequest};

use crate::system::Experiments;

pub async fn create_experiment(
    experiments: &Experiments,
    request: PCreateExperimentRequest,
) -> Result<PCreateExperimentReply> {
    let definition = request.definition
        .ok_or_else(|| anyhow!("No experiment definition has been provided"))?
        .try_into()?;

    let id = experiments
        .launch(definition)
        .await;

    Ok(PCreateExperimentReply {
        id: id.into(),
    })
}