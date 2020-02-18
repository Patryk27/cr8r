use anyhow::*;

use lib_interop::models::DExperimentId;

use super::super::ExperimentStoreActor;

pub fn delete(actor: &mut ExperimentStoreActor, id: DExperimentId) -> Result<()> {
    let experiment = actor.experiments
        .remove(&id)
        .ok_or_else(|| anyhow!("Experiment [id={}] could not be found", id))?;

    experiment.stop();

    Ok(())
}