use crate::system::Runner;

use super::super::RunnerStoreActor;

pub fn find_all(actor: &mut RunnerStoreActor) -> Vec<Runner> {
    actor.runners
        .values()
        .cloned()
        .collect()
}