use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::{convert, Error, Result};
use crate::domain::DExperimentId;
use crate::proto::core::PRunnerStatus;

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
    type Error = Error;

    fn try_from(PRunnerStatus { ty }: PRunnerStatus) -> Result<Self> {
        use crate::proto::core::p_runner_status::*;

        Ok(match convert!(ty?) {
            Ty::Idle(PIdle { since }) => {
                DRunnerStatus::Idle {
                    since: convert!(since as DateTime),
                }
            }

            Ty::Working(PWorking { since, .. }) => {
                DRunnerStatus::Working {
                    since: convert!(since as DateTime),
                    experiment_id: convert!(since as _),
                }
            }

            Ty::Zombie(PZombie { since }) => {
                DRunnerStatus::Zombie {
                    since: convert!(since as DateTime),
                }
            }
        })
    }
}

impl Into<PRunnerStatus> for DRunnerStatus {
    fn into(self) -> PRunnerStatus {
        use crate::proto::core::p_runner_status::*;

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