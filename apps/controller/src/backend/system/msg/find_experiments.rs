use crate::backend::Experiment;
use crate::backend::system::SystemActor;

pub fn process(actor: &mut SystemActor) -> Vec<Experiment> {
    actor.experiments.all()
}