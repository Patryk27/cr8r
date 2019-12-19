use crate::backend::Experiment;
use crate::backend::system::SystemActor;

pub fn find_experiments(actor: &mut SystemActor) -> Vec<Experiment> {
    actor.experiments.all()
}