use std::convert::TryInto;

use lib_interop::protocol::for_client::{PCreateExperimentReply, PCreateExperimentRequest};

use crate::backend::{Result, System};

pub async fn create_experiment(system: &System, request: PCreateExperimentRequest) -> Result<PCreateExperimentReply> {
    let def = request.experiment_def
        .ok_or("No experiment definition has been provided")?
        .try_into()?;

    let id = system
        .create_experiment(def)
        .await?;

    Ok(PCreateExperimentReply {
        id: id.into(),
    })
}