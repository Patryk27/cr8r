use std::convert::TryInto;

use anyhow::*;

use lib_interop::proto::services::{PAddEventReply, PAddEventRequest};

use crate::system::Experiments;

pub async fn add_event(
    experiments: &Experiments,
    request: PAddEventRequest,
) -> Result<PAddEventReply> {
    let event = request.event
        .ok_or_else(|| anyhow!("No event has been provided"))?
        .try_into()?;

    let experiment_id = request.experiment_id.into();
    let runner_id = request.runner_id.into();

    let experiment = experiments
        .find_one(experiment_id)
        .await?;

    experiment
        .add_event(runner_id, event)
        .await?;

    Ok(Default::default())
}