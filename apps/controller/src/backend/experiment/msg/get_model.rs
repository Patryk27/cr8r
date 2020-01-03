use lib_interop::domain::{DExperiment, DExperimentStatus};

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};

pub fn get_model(actor: &mut ExperimentActor) -> DExperiment {
    let status = match &actor.status {
        ExperimentStatus::Idle { since } => {
            DExperimentStatus::Idle {
                since: since.to_owned(),
            }
        }

        ExperimentStatus::Running { since, last_heartbeat_at, completed_jobs, .. } => {
            DExperimentStatus::Running {
                since: since.to_owned(),
                last_heartbeat_at: last_heartbeat_at.to_owned(),
                completed_jobs: *completed_jobs,
                total_jobs: actor.jobs.len() as _,
            }
        }

        ExperimentStatus::Completed { since, result, .. } => {
            DExperimentStatus::Completed {
                since: since.to_owned(),
                result: result.to_owned(),
            }
        }

        ExperimentStatus::Zombie { since, .. } => {
            DExperimentStatus::Zombie {
                since: since.to_owned(),
            }
        }
    };

    DExperiment {
        id: actor.id.to_owned(),
        created_at: actor.created_at.to_owned(),
        status,
    }
}