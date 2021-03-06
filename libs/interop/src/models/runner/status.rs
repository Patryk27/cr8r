use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::conv;
use crate::models::{DExperimentId, ModelError, ModelResult};
use crate::proto::models::PRunnerStatus;

#[derive(Clone, Debug)]
pub enum DRunnerStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Working {
        since: DateTime<Utc>,
        experiment_id: DExperimentId,
    },

    Zombie {
        since: DateTime<Utc>,
    },
}

impl TryFrom<PRunnerStatus> for DRunnerStatus {
    type Error = ModelError;

    fn try_from(PRunnerStatus { ty }: PRunnerStatus) -> ModelResult<Self> {
        use crate::proto::models::p_runner_status::*;

        Ok(match conv!(ty?) {
            Ty::Idle(PIdle { since }) => {
                DRunnerStatus::Idle {
                    since: conv!(since as DateTime),
                }
            }

            Ty::Working(PWorking { since, experiment_id }) => {
                DRunnerStatus::Working {
                    since: conv!(since as DateTime),
                    experiment_id: conv!(experiment_id as _),
                }
            }

            Ty::Zombie(PZombie { since }) => {
                DRunnerStatus::Zombie {
                    since: conv!(since as DateTime),
                }
            }
        })
    }
}

impl Into<PRunnerStatus> for DRunnerStatus {
    fn into(self) -> PRunnerStatus {
        use crate::proto::models::p_runner_status::*;

        let ty = match self {
            DRunnerStatus::Idle { since } => {
                Ty::Idle(PIdle {
                    since: since.to_rfc3339(),
                })
            }

            DRunnerStatus::Working { since, experiment_id } => {
                Ty::Working(PWorking {
                    since: since.to_rfc3339(),
                    experiment_id: experiment_id.into(),
                })
            }

            DRunnerStatus::Zombie { since } => {
                Ty::Zombie(PZombie {
                    since: since.to_rfc3339(),
                })
            }
        };

        PRunnerStatus { ty: Some(ty) }
    }
}