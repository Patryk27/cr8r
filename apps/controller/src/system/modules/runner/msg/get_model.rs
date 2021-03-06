use lib_interop::models::{DRunner, DRunnerStatus};

use super::super::{RunnerActor, RunnerStatus};

pub fn get_model(actor: &RunnerActor) -> DRunner {
    let status = match &actor.status {
        RunnerStatus::Idle { since } => {
            DRunnerStatus::Idle {
                since: since.to_owned(),
            }
        }

        RunnerStatus::Working { since, experiment_id, .. } => {
            DRunnerStatus::Working {
                since: since.to_owned(),
                experiment_id: experiment_id.to_owned(),
            }
        }

        RunnerStatus::Zombie { since, .. } => {
            DRunnerStatus::Zombie {
                since: since.to_owned(),
            }
        }
    };

    DRunner {
        id: actor.id,
        name: actor.name.clone(),
        joined_at: actor.joined_at,
        last_heartbeat_at: actor.last_heartbeat_at,
        status,
    }
}