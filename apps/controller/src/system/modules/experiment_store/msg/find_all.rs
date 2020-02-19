use crate::system::Experiment;

use super::super::ExperimentStoreActor;

pub fn find_all(actor: &mut ExperimentStoreActor) -> Vec<Experiment> {
    actor.experiments
        .values()
        .cloned()
        .collect()
}