use lib_protocol::core::{PAssignment, PRunnerId};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub async fn process(actor: &mut SystemActor, runner: PRunnerId) -> Result<Option<PAssignment>> {
    if let Some(experiment) = actor.experiments.take() {
        let assignment = experiment
            .start(runner)
            .await?;

        Ok(Some(assignment))
    } else {
        Ok(None)
    }
}