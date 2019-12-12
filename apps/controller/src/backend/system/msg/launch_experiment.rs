use lib_protocol::core::p_experiment_definition::Op as PExperimentDefinitionOp;
use lib_protocol::core::PExperimentId;

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub fn process(actor: &mut SystemActor, definition: PExperimentDefinitionOp) -> Result<PExperimentId> {
    let scenarios = actor.compiler.compile(&definition)?;
    let id = actor.experiments.create(scenarios);

    Ok(id)
}
