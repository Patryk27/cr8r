use lib_interop::contract::{CRunner, CRunnerStatus};

use crate::backend::runner::{RunnerActor, RunnerStatus};

pub fn get_model(actor: &mut RunnerActor) -> CRunner {
    let status = match &actor.status {
        RunnerStatus::Idle { since } => {
            CRunnerStatus::Idle {
                since: since.to_owned(),
            }
        }

        RunnerStatus::Working { since, experiment_id, .. } => {
            CRunnerStatus::Working {
                since: since.to_owned(),
                experiment_id: experiment_id.to_owned(),
            }
        }

        RunnerStatus::Zombie { since } => {
            CRunnerStatus::Zombie {
                since: since.to_owned(),
            }
        }
    };

    CRunner {
        id: actor.id.clone(),
        name: actor.name.clone(),
        joined_at: actor.joined_at,
        last_heartbeat_at: actor.last_heartbeat_at,
        status,
    }
}