use chrono::{DateTime, Utc};

use crate::contract::CExperimentId;
use crate::protocol::core::p_runner::PStatus;

#[derive(Clone, Debug)]
pub enum CRunnerStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Working {
        since: DateTime<Utc>,
        experiment_id: CExperimentId,
    },

    Zombie {
        since: DateTime<Utc>,
    },
}

impl Into<PStatus> for CRunnerStatus {
    fn into(self) -> PStatus {
        use crate::protocol::core::p_runner::p_status::*;

        let op = match self {
            CRunnerStatus::Idle { since } => {
                Op::Idle(PIdle {
                    since: since.to_rfc3339(),
                })
            }

            CRunnerStatus::Working { since, experiment_id } => {
                Op::Working(PWorking {
                    since: since.to_rfc3339(),
                    experiment_id: experiment_id.into(),
                })
            }

            CRunnerStatus::Zombie { since } => {
                Op::Zombie(PZombie {
                    since: since.to_rfc3339(),
                })
            }
        };

        PStatus {
            op: Some(op),
        }
    }
}