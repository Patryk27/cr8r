use lib_interop::domain::{DAssignment, DRunnerId};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub async fn get_assignment(actor: &mut SystemActor, runner_id: DRunnerId) -> Result<Option<DAssignment>> {
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