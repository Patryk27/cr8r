use std::sync::Arc;

use anyhow::*;
use tokio::sync::mpsc;

use lib_core_channel::URx;
use lib_interop::domain::DReport;

use super::super::{ExperimentActor, ExperimentStatus};

pub fn watch(actor: &mut ExperimentActor) -> Result<URx<Arc<DReport>>> {
    match actor.status {
        ExperimentStatus::Idle { .. } | ExperimentStatus::Running { .. } => {
            let (tx, rx) = mpsc::unbounded_channel();

            actor.watchers.push(tx);

            Ok(rx)
        }

        ExperimentStatus::Completed { .. } => {
            Err(anyhow!("This experiment has been already completed"))
        }

        ExperimentStatus::Stopped { .. } => {
            Err(anyhow!("This experiment has been already stopped"))
        }
    }
}