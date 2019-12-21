use lib_interop::contract::{CAssignment, CRunnerId};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub async fn get_assignment(actor: &mut SystemActor, runner_id: CRunnerId) -> Result<Option<CAssignment>> {
    // @todo notify runner about the experiment

    if let Some(experiment) = actor.experiments.pop() {
        let assignment = experiment
            .start(runner_id)
            .await?;

        Ok(Some(assignment))
    } else {
        Ok(None)
    }
}