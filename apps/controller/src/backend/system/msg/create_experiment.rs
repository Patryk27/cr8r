use anyhow::*;

use lib_interop::domain::{DDefinition, DExperimentId};

use crate::backend::system::SystemActor;

pub fn create_experiment(actor: &mut SystemActor, definition: DDefinition) -> Result<DExperimentId> {
    let jobs = actor.compiler.compile(&definition);
    let id = actor.experiments.create(jobs);

    Ok(id)
}
