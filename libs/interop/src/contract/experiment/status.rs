use std::convert::TryFrom;
use std::result;

use chrono::{DateTime, Utc};

use crate::{convert, Error, Result};
use crate::protocol::core::PExperimentStatus;

#[derive(Clone, Debug)]
pub enum CExperimentStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat_at: DateTime<Utc>,
        completed_jobs: u32,
        total_jobs: u32,
    },

    Completed {
        since: DateTime<Utc>,
        result: result::Result<(), String>,
    },

    Zombie {
        since: DateTime<Utc>,
    },
}

impl TryFrom<PExperimentStatus> for CExperimentStatus {
    type Error = Error;

    fn try_from(PExperimentStatus { ty }: PExperimentStatus) -> Result<Self> {
        use crate::protocol::core::p_experiment_status::*;

        Ok(match convert!(ty?) {
            Ty::Idle(PIdle { since }) => {
                CExperimentStatus::Idle {
                    since: convert!(since as DateTime),
                }
            }

            Ty::Running(PRunning { since, last_heartbeat_at, completed_jobs, total_jobs }) => {
                CExperimentStatus::Running {
                    since: convert!(since as DateTime),
                    last_heartbeat_at: convert!(last_heartbeat_at as DateTime),
                    completed_jobs,
                    total_jobs,
                }
            }

            Ty::Completed(PCompleted { since, success, cause }) => {
                let result = if success {
                    Ok(())
                } else {
                    Err(cause)
                };

                CExperimentStatus::Completed {
                    since: convert!(since as DateTime),
                    result,
                }
            }

            Ty::Zombie(PZombie { since }) => {
                CExperimentStatus::Zombie {
                    since: convert!(since as DateTime),
                }
            }
        })
    }
}

impl Into<PExperimentStatus> for CExperimentStatus {
    fn into(self) -> PExperimentStatus {
        use crate::protocol::core::p_experiment_status::*;

        let ty = match self {
            CExperimentStatus::Idle { since } => {
                Ty::Idle(PIdle {
                    since: since.to_rfc3339(),
                })
            }

            CExperimentStatus::Running { since, last_heartbeat_at, completed_jobs, total_jobs } => {
                Ty::Running(PRunning {
                    since: since.to_rfc3339(),
                    last_heartbeat_at: last_heartbeat_at.to_rfc3339(),
                    completed_jobs,
                    total_jobs,
                })
            }

            CExperimentStatus::Completed { since, result } => {
                Ty::Completed(PCompleted {
                    since: since.to_rfc3339(),
                    success: result.is_ok(),
                    cause: result.err().unwrap_or_default(),
                })
            }

            CExperimentStatus::Zombie { since } => {
                Ty::Zombie(PZombie {
                    since: since.to_rfc3339(),
                })
            }
        };

        PExperimentStatus { ty: Some(ty) }
    }
}