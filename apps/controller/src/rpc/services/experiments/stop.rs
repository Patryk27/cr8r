use anyhow::*;

use lib_interop::proto::services::{PStopExperimentReply, PStopExperimentRequest};

use crate::system::ExperimentStore;

pub async fn stop_experiment(
    experiment_store: &ExperimentStore,
    request: PStopExperimentRequest,
) -> Result<PStopExperimentReply> {
    let id = request.id.into();

    let experiment = experiment_store
        .find_one(id)
        .await?;

    experiment.stop();

    Ok(PStopExperimentReply {})
}