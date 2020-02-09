use anyhow::*;
use log::*;

use lib_interop::domain::{DRunnerId, DRunnerName};

use crate::system::Runner;
use crate::system::runners::actor::RunnersActor;

pub fn register(actor: &mut RunnersActor, name: DRunnerName) -> Result<DRunnerId> {
    if let Some(id) = actor.index.get_by_right(&name) {
        warn!("A new runner tries to join under a name that's already taken: {}", name);

        // @todo if the currently present runner is not a zombie, don't kick it out
        warn!("Decision: Current runner (id={}) will be kicked out", id);

        let runner = &actor.runners[id];

        runner.kill();

        actor.runners.remove(&id);
    }

    let id = actor.next_id.inc();

    info!("Got a new runner: id={}, name={}", id, name);

    let runner = Runner::new(
        id, name.clone(),
    );

    actor.index.insert(id, name);
    actor.runners.insert(id, runner);

    Ok(id)
}