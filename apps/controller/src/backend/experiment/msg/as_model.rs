use lib_protocol::core::p_experiment::p_status::*;
use lib_protocol::core::p_experiment::PStatus;
use lib_protocol::core::PExperiment;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};

pub fn process(actor: &mut ExperimentActor) -> PExperiment {
    let status = match &actor.status {
        ExperimentStatus::AwaitingRunner { since } => {
            Op::AwaitingRunner(PAwaitingRunner {
                since: since.to_rfc3339(),
            })
        }

        ExperimentStatus::Running { since, last_heartbeat_at, completed_scenarios, .. } => {
            Op::Running(PRunning {
                since: since.to_rfc3339(),
                last_heartbeat_at: last_heartbeat_at.to_rfc3339(),
                completed_scenarios: *completed_scenarios,
            })
        }

        ExperimentStatus::Completed { since, success } => {
            Op::Completed(PCompleted {
                since: since.to_rfc3339(),
                success: *success,
            })
        }

        ExperimentStatus::Aborted { since } => {
            Op::Aborted(PAborted {
                since: since.to_rfc3339(),
            })
        }

        ExperimentStatus::Zombie { since, .. } => {
            Op::Zombie(PZombie {
                since: since.to_rfc3339(),
            })
        }
    };

    PExperiment {
        id: actor.experiment.clone(),
        created_at: actor.created_at.to_rfc3339(),
        scenario_count: actor.scenarios.len() as u32,

        status: Some(PStatus {
            op: Some(status),
        }),
    }
}