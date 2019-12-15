use std::sync::Arc;

use futures_channel::mpsc;

use lib_protocol::core::PExperimentReport;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn process(actor: &mut ExperimentActor) -> Result<mpsc::UnboundedReceiver<Arc<PExperimentReport>>> {
    match actor.status {
        ExperimentStatus::AwaitingRunner { .. } | ExperimentStatus::Running { .. } | ExperimentStatus::Zombie { .. } => {
            let (tx, rx) = mpsc::unbounded();

            actor.watchers.push(tx);

            Ok(rx)
        }

        ExperimentStatus::Completed { .. } => {
            Err("This experiment has been already completed".into())
        }

        ExperimentStatus::Aborted { .. } => {
            Err("This experiment has been already aborted".into())
        }
    }
}