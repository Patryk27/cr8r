use lib_interop::contract::{CExperimentDef, CExperimentId};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub fn create_experiment(actor: &mut SystemActor, def: CExperimentDef) -> Result<CExperimentId> {
    let program = actor.compiler.compile(&def);
    let id = actor.experiments.create(program);

    Ok(id)
}
