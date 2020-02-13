use anyhow::*;

use lib_interop::domain::DExperimentId;

use super::super::ExperimentsActor;

pub fn delete(actor: &mut ExperimentsActor, id: DExperimentId) -> Result<()> {
    let experiment = actor.experiments
        .remove(&id)
        .ok_or_else(|| anyhow!("Experiment [id={}] could not be found", id))?;

    experiment.stop();

    Ok(())
}