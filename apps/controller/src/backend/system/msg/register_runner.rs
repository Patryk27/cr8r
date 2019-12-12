use lib_protocol::core::{PRunnerId, PRunnerName};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub fn process(actor: &mut SystemActor, name: PRunnerName) -> Result<PRunnerId> {
    if let Some(id) = actor.runners.name_to_id(&name).map(ToOwned::to_owned) {
        let runner = actor.runners
            .get(&id)
            .unwrap();

        runner.kill();

        actor.runners.remove(&id);
    }

    actor.runners.create(name)
}