use log::*;

use lib_protocol::core::{PRunnerId, PRunnerName};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub fn process(actor: &mut SystemActor, name: PRunnerName) -> Result<PRunnerId> {
    if let Some(id) = actor.runners.name_to_id(&name).map(ToOwned::to_owned) {
        warn!("A new runner tries to join under a name that's already taken: {}", name);

        let runner = actor.runners
            .get(&id)
            .unwrap();

        // @todo if currently present runner is not a zombie, forbid kicking it out
        warn!("Decision: Current runner will be kicked out");

        runner.kill();

        actor.runners.remove(&id);
    }

    info!("Runner joined: {}", name);
    actor.runners.create(name)
}