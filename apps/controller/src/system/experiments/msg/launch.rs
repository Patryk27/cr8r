use lib_interop::domain::{DDefinition, DExperimentId};

use crate::system::Experiment;

use super::super::ExperimentsActor;

pub fn launch(actor: &mut ExperimentsActor, definition: DDefinition) -> DExperimentId {
    let id = actor.next_id.inc();
    let jobs = actor.compiler.compile(&definition);

    let experiment = Experiment::new(id, jobs);

    actor.experiments.insert(id, experiment);
    actor.pending_ids.push_back(id);

    id
}