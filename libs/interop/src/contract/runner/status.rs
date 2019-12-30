use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::{convert, Error, Result};
use crate::contract::CExperimentId;
use crate::protocol::core::PRunnerStatus;

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

impl TryFrom<PRunnerStatus> for CRunnerStatus {
    type Error = Error;

    fn try_from(PRunnerStatus { ty }: PRunnerStatus) -> Result<Self> {
        use crate::protocol::core::p_runner_status::*;

        Ok(match convert!(ty?) {
            Ty::Idle(PIdle { since }) => {
                CRunnerStatus::Idle {
                    since: convert!(since as DateTime),
                }
            }

            Ty::Working(PWorking { since, .. }) => {
                CRunnerStatus::Working {
                    since: convert!(since as DateTime),
                    experiment_id: convert!(since as _),
                }
            }

            Ty::Zombie(PZombie { since }) => {
                CRunnerStatus::Zombie {
                    since: convert!(since as DateTime),
                }
            }
        })
    }
}

impl Into<PRunnerStatus> for CRunnerStatus {
    fn into(self) -> PRunnerStatus {
        use crate::protocol::core::p_runner_status::*;

        let ty = match self {
            CRunnerStatus::Idle { since } => {
                Ty::Idle(PIdle {
                    since: since.to_rfc3339(),
                })
            }

            CRunnerStatus::Working { since, experiment_id } => {
                Ty::Working(PWorking {
                    since: since.to_rfc3339(),
                    experiment_id: experiment_id.into(),
                })
            }

            CRunnerStatus::Zombie { since } => {
                Ty::Zombie(PZombie {
                    since: since.to_rfc3339(),
                })
            }
        };

        PRunnerStatus { ty: Some(ty) }
    }
}