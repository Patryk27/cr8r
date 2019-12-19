use lib_protocol::core::p_experiment::p_status::*;
use lib_protocol::core::p_experiment::PStatus;
use lib_protocol::core::PExperiment;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};

pub fn get_model(actor: &mut ExperimentActor) -> PExperiment {
    let status = match &actor.status {
        ExperimentStatus::Idle { since } => {
            Op::Idle(PIdle {
                since: since.to_rfc3339(),
            })
        }

        ExperimentStatus::Running { since, last_heartbeat_at, completed_steps, .. } => {
            Op::Running(PRunning {
                since: since.to_rfc3339(),
                last_heartbeat_at: last_heartbeat_at.to_rfc3339(),
                completed_steps: *completed_steps,
            })
        }

        ExperimentStatus::Completed { since, result, .. } => {
            let cause = result
                .as_ref()
                .err()
                .map(ToOwned::to_owned)
                .unwrap_or_default();

            Op::Completed(PCompleted {
                since: since.to_rfc3339(),
                success: result.is_ok(),
                cause,
            })
        }

        ExperimentStatus::Zombie { since, .. } => {
            Op::Zombie(PZombie {
                since: since.to_rfc3339(),
            })
        }
    };

    PExperiment {
        id: actor.id.clone(),
        system: actor.system.clone(),
        toolchain: actor.toolchain.clone(),
        steps: actor.steps.clone(),
        created_at: actor.created_at.to_rfc3339(),

        status: Some(PStatus {
            op: Some(status),
        }),
    }
}