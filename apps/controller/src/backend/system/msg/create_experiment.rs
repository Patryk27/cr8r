use lib_interop::contract::{CExperimentDefinition, CExperimentId};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub fn create_experiment(actor: &mut SystemActor, def: CExperimentDefinition) -> Result<CExperimentId> {
    let jobs = actor.compiler.compile(&def);
    let id = actor.experiments.create(jobs);

    Ok(id)
}
