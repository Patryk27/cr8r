use std::convert::TryInto;

use anyhow::*;

use lib_interop::proto::controller::{PCreateExperimentReply, PCreateExperimentRequest};

use crate::system::System;

pub async fn create_experiment(system: &System, request: PCreateExperimentRequest) -> Result<PCreateExperimentReply> {
    let definition = request.definition
        .ok_or_else(|| anyhow!("No experiment definition has been provided"))?
        .try_into()?;

    let id = system
        .experiments
        .launch(definition)
        .await;

    Ok(PCreateExperimentReply {
        id: id.into(),
    })
}