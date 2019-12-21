use std::convert::TryInto;

use lib_interop::protocol::for_runner::{PAddExperimentEventReply, PAddExperimentEventRequest};

use crate::backend::{Result, System};

pub async fn add_experiment_event(
    system: &System,
    request: PAddExperimentEventRequest,
) -> Result<PAddExperimentEventReply> {
    let event = request.experiment_event
        .ok_or("No event has been provided")?
        .try_into()?;

    let experiment_id = request.experiment_id.into();
    let runner_id = request.runner_id.into();

    system
        .find_experiment(experiment_id)
        .await?
        .add_event(runner_id, event).await?;

    Ok(PAddExperimentEventReply::default())
}