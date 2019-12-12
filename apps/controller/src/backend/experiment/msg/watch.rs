use crate::backend::experiment::ExperimentActor;
use crate::backend::ExperimentWatcher;

pub fn process(actor: &mut ExperimentActor) -> ExperimentWatcher {
    if let Some(mut watcher) = actor.watcher.take() {
        watcher.kill();
    }

    let watcher = ExperimentWatcher::spawn();

    // @todo allow handling many watchers at once
    actor.watcher = Some(watcher.clone());

    watcher
}