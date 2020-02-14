use anyhow::*;
use chrono::Utc;

use lib_interop::domain::DRunnerId;

use super::super::{ExperimentActor, ExperimentStatus};

pub fn claim(actor: &mut ExperimentActor, runner: DRunnerId) -> Result<()> {
    match &actor.status {
        ExperimentStatus::Idle { .. } => {
            actor.status = ExperimentStatus::Running {
                since: Utc::now(),
                last_heartbeat_at: Utc::now(),
                runner,
                events: Vec::new(),
                reports: Vec::new(),
                completed_jobs: 0,
            };

            Ok(())
        }

        ExperimentStatus::Running { .. } => {
            Err(anyhow!("This experiment is already running"))
        }

        ExperimentStatus::Completed { .. } => {
            Err(anyhow!("This experiment has been already completed"))
        }

        ExperimentStatus::Stopped { .. } => {
            Err(anyhow!("This experiment has been already stopped"))
        }
    }
}