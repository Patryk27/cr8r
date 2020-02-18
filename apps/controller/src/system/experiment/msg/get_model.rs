use lib_interop::models::{DExperiment, DExperimentStatus};

use super::super::{ExperimentActor, ExperimentStatus};

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

        ExperimentStatus::Stopped { since } => {
            DExperimentStatus::Stopped {
                since: since.to_owned(),
            }
        }
    };

    DExperiment {
        id: actor.id,
        created_at: actor.created_at,
        status,
    }
}