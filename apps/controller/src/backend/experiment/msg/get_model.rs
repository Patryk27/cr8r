use lib_interop::contract::{CExperiment, CExperimentStatus};

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};

pub fn get_model(actor: &mut ExperimentActor) -> CExperiment {
    let status = match &actor.status {
        ExperimentStatus::Idle { since } => {
            CExperimentStatus::Idle {
                since: since.to_owned(),
            }
        }

        ExperimentStatus::Running { since, last_heartbeat_at, completed_steps, .. } => {
            CExperimentStatus::Running {
                since: since.to_owned(),
                last_heartbeat_at: last_heartbeat_at.to_owned(),
                completed_steps: *completed_steps,
            }
        }

        ExperimentStatus::Completed { since, result, .. } => {
            CExperimentStatus::Completed {
                since: since.to_owned(),
                result: result.to_owned(),
            }
        }

        ExperimentStatus::Zombie { since, .. } => {
            CExperimentStatus::Zombie {
                since: since.to_owned(),
            }
        }
    };

    CExperiment {
        id: actor.id.to_owned(),
        created_at: actor.created_at.to_owned(),
        status,
    }
}