use lib_interop::domain::{DExperimentDefinition, DExperimentId};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub fn create_experiment(actor: &mut SystemActor, def: DExperimentDefinition) -> Result<DExperimentId> {
    let jobs = actor.compiler.compile(&def);
    let id = actor.experiments.create(jobs);

    Ok(id)
}
