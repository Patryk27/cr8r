use std::sync::Arc;

use tokio::sync::mpsc;

use anyhow::anyhow;
use lib_interop::domain::DReport;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn watch(actor: &mut ExperimentActor) -> Result<mpsc::UnboundedReceiver<Arc<DReport>>> {
    match actor.status {
        ExperimentStatus::Idle { .. } | ExperimentStatus::Running { .. } | ExperimentStatus::Zombie { .. } => {
            let (tx, rx) = mpsc::unbounded_channel();

            actor.watchers.push(tx);

            Ok(rx)
        }

        ExperimentStatus::Completed { .. } => {
            Err(anyhow!("This experiment has been already completed"))
        }
    }
}