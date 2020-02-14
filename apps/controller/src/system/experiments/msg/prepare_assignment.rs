use anyhow::*;

use lib_interop::domain::{DExperimentId, DRunnerId};

use super::super::ExperimentsActor;

pub async fn prepare_assignment(actor: &mut ExperimentsActor, runner_id: DRunnerId) -> Result<Option<DExperimentId>> {
    if let Some(experiment_id) = actor.pending_ids.pop_front() {
        actor
            .experiments[&experiment_id]
            .claim(runner_id)
            .await?;

        Ok(Some(experiment_id))
    } else {
        Ok(None)
    }
}