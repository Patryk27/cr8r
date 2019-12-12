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

        ExperimentStatus::Running { since, runner, completed_scenarios, total_scenarios, .. } => {
            Op::Running(PRunning {
                since: since.to_rfc3339(),
                runner_id: runner.to_owned(),
                completed_scenarios: *completed_scenarios,
                total_scenarios: *total_scenarios,
            })
        }

        ExperimentStatus::Completed { since } => {
            Op::Completed(PCompleted {
                since: since.to_rfc3339(),
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
        heartbeaten_at: "@todo".to_string(),

        status: Some(PStatus {
            op: Some(status),
        }),
    }
}