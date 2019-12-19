use lib_protocol::core::p_experiment_def::Op as PExperimentDefOp;
use lib_protocol::core::PExperimentId;

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub fn launch_experiment(actor: &mut SystemActor, experiment_def: PExperimentDefOp) -> Result<PExperimentId> {
    let id = actor.experiments.create(
        actor.compiler.compile(&experiment_def)?
    );

    Ok(id)
}
