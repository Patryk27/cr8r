use std::sync::Arc;

use futures_channel::mpsc;

use lib_protocol::core::PExperimentReport;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn watch(actor: &mut ExperimentActor) -> Result<mpsc::UnboundedReceiver<Arc<PExperimentReport>>> {
    match actor.status {
        ExperimentStatus::Idle { .. } | ExperimentStatus::Running { .. } | ExperimentStatus::Zombie { .. } => {
            let (tx, rx) = mpsc::unbounded();

            actor.watchers.push(tx);

            Ok(rx)
        }

        ExperimentStatus::Completed { .. } => {
            Err("This experiment has been already completed".into())
        }
    }
}