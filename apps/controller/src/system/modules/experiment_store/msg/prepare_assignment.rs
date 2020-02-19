use anyhow::*;

use lib_interop::models::{DExperimentId, DRunnerId};

use super::super::ExperimentStoreActor;

pub async fn prepare_assignment(actor: &mut ExperimentStoreActor, runner_id: DRunnerId) -> Result<Option<DExperimentId>> {
    if let Some(experiment_id) = actor.waiting_experiments.pop_front() {
        actor
            .experiments[&experiment_id]
            .claim(runner_id).await?;

        Ok(Some(experiment_id))
    } else {
        Ok(None)
    }
}