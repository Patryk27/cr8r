use anyhow::*;

use lib_interop::proto::services::{PStopExperimentReply, PStopExperimentRequest};

use crate::system::System;

pub async fn stop_experiment(system: &System, request: PStopExperimentRequest) -> Result<PStopExperimentReply> {
    let id = request.id.into();

    let experiment = system.experiments
        .find_one(id)
        .await?;

    experiment.stop();

    Ok(PStopExperimentReply {})
}