use anyhow::*;

use lib_interop::models::DExperimentId;

use crate::system::Experiment;

use super::super::ExperimentStoreActor;

pub fn find_one(actor: &mut ExperimentStoreActor, id: DExperimentId) -> Result<Experiment> {
    actor.experiments
        .get(&id)
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow!("Experiment `{}` does not exist", id))
}