use anyhow::*;

use lib_interop::domain::DExperimentId;

use crate::system::Experiment;

use super::super::ExperimentsActor;

pub fn find_one(actor: &mut ExperimentsActor, id: DExperimentId) -> Result<Experiment> {
    actor.experiments
        .get(&id)
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow!("Experiment `{}` does not exist", id))
}