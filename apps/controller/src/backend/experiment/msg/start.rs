use chrono::Utc;

use lib_protocol::core::{PAssignment, PRunnerId};

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn process(actor: &mut ExperimentActor, runner: PRunnerId) -> Result<PAssignment> {
    match &actor.status {
        ExperimentStatus::AwaitingRunner { .. } => {
            actor.status = ExperimentStatus::Running {
                since: Utc::now(),
                last_heartbeat_at: Utc::now(),
                runner,
                reports: Vec::new(),
                completed_scenarios: 0,
            };

            Ok(PAssignment {
                experiment_id: actor.experiment.clone(),
                experiment_scenarios: actor.scenarios.clone(),
            })
        }

        ExperimentStatus::Running { runner, .. } => {
            Err(format!(
                "This experiment is already running on runner `{}` and thus cannot be re-claimed yet. If the runner's crashed, please wait a few minutes before trying again.",
                runner,
            ).into())
        }

        ExperimentStatus::Completed { .. } => {
            Err("This experiment has been already completed - if you want to re-start it, please create a new one".into())
        }

        ExperimentStatus::Aborted { .. } => {
            Err("This experiment has been aborted - if you want to re-start it, please create a new one".into())
        }

        ExperimentStatus::Zombie { .. } => {
            unimplemented!()
        }
    }
}