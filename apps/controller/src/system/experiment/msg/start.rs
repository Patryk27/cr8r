use anyhow::*;
use chrono::Utc;

use lib_interop::domain::{DAssignment, DRunnerId};

use super::super::{ExperimentActor, ExperimentStatus};

pub fn start(actor: &mut ExperimentActor, runner: DRunnerId) -> Result<DAssignment> {
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

            Ok(DAssignment {
                experiment: super::get_model::get_model(actor),
                jobs: actor.jobs.clone(),
            })
        }

        ExperimentStatus::Running { runner, .. } => {
            Err(anyhow!(
                "This experiment is already running on runner `{}` and cannot be reclaimed yet. \
                 If the runner's crashed, please wait a few minutes before trying again.",
                runner,
            ))
        }

        ExperimentStatus::Completed { .. } => {
            Err(anyhow!(
                "This experiment has been already completed - if you want to restart it, please create a new one",
            ))
        }

        ExperimentStatus::Stopped { .. } => {
            Err(anyhow!("This experiment has been already stopped"))
        }
    }
}