use crate::backend::{ExperimentWatcher, Result};
use crate::backend::experiment::{ExperimentActor, ExperimentStatus};

pub fn process(actor: &mut ExperimentActor) -> Result<ExperimentWatcher> {
    match actor.status {
        ExperimentStatus::AwaitingRunner { .. } | ExperimentStatus::Running { .. } | ExperimentStatus::Zombie { .. } => {
            if let Some(mut watcher) = actor.watcher.take() {
                watcher.kill();
            }

            let watcher = ExperimentWatcher::spawn();

            // @todo allow handling many watchers at once
            actor.watcher = Some(watcher.clone());

            Ok(watcher)
        }

        ExperimentStatus::Completed { .. } => {
            Err("This experiment has been already completed".into())
        }

        ExperimentStatus::Aborted { .. } => {
            Err("This experiment has been already aborted".into())
        }
    }
}