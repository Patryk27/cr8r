use std::convert::TryInto;

use anyhow::anyhow;
use lib_interop::proto::controller::{PCreateExperimentReply, PCreateExperimentRequest};

use crate::backend::{Result, System};

pub async fn create_experiment(system: &System, request: PCreateExperimentRequest) -> Result<PCreateExperimentReply> {
    let def = request.experiment_definition
        .ok_or_else(|| anyhow!("No experiment definition has been provided"))?
        .try_into()?;

    let id = system
        .create_experiment(def)
        .await?;

    Ok(PCreateExperimentReply {
        id: id.into(),
    })
}