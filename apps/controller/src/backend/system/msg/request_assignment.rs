use lib_protocol::core::{PAssignment, PRunnerId};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub async fn process(actor: &mut SystemActor, runner: PRunnerId) -> Result<Option<PAssignment>> {
    // @todo translate `PRunnerId` into `Runner`, notify runner about the experiment

    if let Some(experiment) = actor.experiments.pop() {
        let assignment = experiment
            .start(runner)
            .await?;

        Ok(Some(assignment))
    } else {
        Ok(None)
    }
}