use crate::backend::Runner;
use crate::backend::system::SystemActor;

pub fn process(actor: &mut SystemActor) -> Vec<Runner> {
    actor.runners
        .all()
        .into_iter()
        .map(ToOwned::to_owned)
        .collect()
}