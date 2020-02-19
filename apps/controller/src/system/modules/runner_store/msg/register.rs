use std::sync::Arc;

use anyhow::*;
use log::*;

use lib_interop::models::{DRunnerId, DRunnerName};

use crate::system::{Runner, SystemEvent};

use super::super::RunnerStoreActor;

pub fn register(actor: &mut RunnerStoreActor, name: DRunnerName) -> Result<DRunnerId> {
    if let Some(id) = actor.index.get_by_right(&name) {
        warn!("A new runner tries to join under a name that's already taken: {}", name);

        // @todo if the currently present runner is not a zombie, don't kick it out
        warn!("Decision: Current runner (id={}) will be kicked out", id);

        let runner = &actor.runners[id];

        runner.kill();

        actor.runners.remove(&id);
    }

    let id = actor.next_id.inc();

    actor.runners.insert(id, Runner::new(
        actor.bus.clone(),
        id,
        name.clone(),
    ));

    actor.bus.emit(SystemEvent::RunnerJoined {
        id,
        name: Arc::new(name.clone()),
    });

    actor.index.insert(id, name);

    Ok(id)
}