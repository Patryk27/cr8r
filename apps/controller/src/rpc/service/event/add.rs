use std::convert::TryInto;

use anyhow::*;

use lib_interop::proto::controller::{PAddEventReply, PAddEventRequest};

use crate::system::System;

pub async fn add_event(
    system: &System,
    request: PAddEventRequest,
) -> Result<PAddEventReply> {
    let event = request.event
        .ok_or_else(|| anyhow!("No event has been provided"))?
        .try_into()?;

    let experiment_id = request.experiment_id.into();
    let runner_id = request.runner_id.into();

    let experiment = system
        .experiments
        .find_one(experiment_id)
        .await?;

    experiment
        .add_event(runner_id, event)
        .await?;

    Ok(Default::default())
}