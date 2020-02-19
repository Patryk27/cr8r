use anyhow::*;

use lib_interop::models::DExperimentId;

use crate::system::SystemEvent;

use super::super::ExperimentStoreActor;

pub fn delete(actor: &mut ExperimentStoreActor, id: DExperimentId) -> Result<()> {
    let experiment = actor.experiments
        .remove(&id)
        .ok_or_else(|| anyhow!("Experiment [id={}] could not be found", id))?;

    experiment.stop();

    actor.bus.emit(SystemEvent::ExperimentDeleted { id });

    Ok(())
}