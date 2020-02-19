use std::convert::TryInto;

use anyhow::*;

use lib_interop::proto::services::{PAddEventReply, PAddEventRequest};

use crate::system::ExperimentStore;

pub async fn add_event(
    experiment_store: &ExperimentStore,
    request: PAddEventRequest,
) -> Result<PAddEventReply> {
    let event = request.event
        .ok_or_else(|| anyhow!("No event has been provided"))?
        .try_into()?;

    let experiment_id = request.experiment_id.into();
    let runner_id = request.runner_id.into();

    experiment_store
        .find_one(experiment_id).await?
        .add_event(runner_id, event).await?;

    Ok(PAddEventReply::default())
}