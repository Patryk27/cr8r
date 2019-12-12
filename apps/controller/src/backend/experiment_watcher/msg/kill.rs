use crate::backend::experiment_watcher::ExperimentWatcherActor;

pub fn process(actor: &mut ExperimentWatcherActor) {
    actor.alive = false;

    if let Some(tx) = actor.pending_get_tx.take() {
        let _ = tx.send(None);
    }
}