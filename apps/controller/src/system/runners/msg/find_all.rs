use crate::system::Runner;

use super::super::RunnersActor;

pub fn find_all(actor: &mut RunnersActor) -> Vec<Runner> {
    actor.runners
        .values()
        .cloned()
        .collect()
}