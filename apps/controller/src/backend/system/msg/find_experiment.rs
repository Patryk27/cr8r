use lib_interop::contract::CExperimentId;

use crate::backend::{Experiment, Result};
use crate::backend::system::SystemActor;

pub fn find_experiment(actor: &mut SystemActor, id: CExperimentId) -> Result<Experiment> {
    actor.experiments.get(&id)
}