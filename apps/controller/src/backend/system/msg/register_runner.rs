use lib_protocol::core::{PRunnerId, PRunnerName};

use crate::backend::Result;
use crate::backend::system::SystemActor;

pub fn process(actor: &mut SystemActor, name: PRunnerName) -> Result<PRunnerId> {
    if let Some(id) = actor.runners.name_to_id(&name) {
        unimplemented!()
    }

    actor.runners.create(name)
}