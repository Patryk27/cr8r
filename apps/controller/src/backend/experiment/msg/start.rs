use chrono::Utc;

use lib_protocol::core::{PAssignment, PRunnerId};

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn start(actor: &mut ExperimentActor, runner: PRunnerId) -> Result<PAssignment> {
    match &actor.status {
        ExperimentStatus::Idle { .. } => {
            actor.status = ExperimentStatus::Running {
                since: Utc::now(),
                last_heartbeat_at: Utc::now(),
                runner,
                events: Vec::new(),
                reports: Vec::new(),
                completed_steps: 0,
            };

            let experiment = super::get_model::get_model(actor);

            Ok(PAssignment {
                experiment: Some(experiment),
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

        ExperimentStatus::Zombie { .. } => {
            unimplemented!()
        }
    }
}