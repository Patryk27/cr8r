use lib_interop::domain::DExperimentId;

use crate::backend::{Experiment, Result};
use crate::backend::system::SystemActor;

pub fn find_experiment(actor: &mut SystemActor, id: DExperimentId) -> Result<Experiment> {
    actor.experiments.get(&id)
}