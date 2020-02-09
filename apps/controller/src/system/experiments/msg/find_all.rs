use crate::system::Experiment;

use super::super::ExperimentsActor;

pub fn find_all(actor: &mut ExperimentsActor) -> Vec<Experiment> {
    actor.experiments
        .values()
        .cloned()
        .collect()
}