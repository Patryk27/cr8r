use anyhow::*;

use lib_interop::proto::services::{PStopExperimentReply, PStopExperimentRequest};

use crate::system::Experiments;

pub async fn stop_experiment(
    experiments: &Experiments,
    request: PStopExperimentRequest,
) -> Result<PStopExperimentReply> {
    let id = request.id.into();

    let experiment = experiments
        .find_one(id)
        .await?;

    experiment.stop();

    Ok(PStopExperimentReply {})
}