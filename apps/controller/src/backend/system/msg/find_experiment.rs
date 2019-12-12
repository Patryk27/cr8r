use lib_protocol::core::PExperimentId;

use crate::backend::{Experiment, Result};
use crate::backend::system::SystemActor;

pub fn process(actor: &mut SystemActor, experiment: PExperimentId) -> Result<Experiment> {
    actor.experiments.get(&experiment)
}