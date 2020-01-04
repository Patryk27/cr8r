use std::convert::TryFrom;
use std::result;

use chrono::{DateTime, Utc};

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::proto::core::PExperimentStatus;

#[derive(Clone, Debug)]
pub enum DExperimentStatus {
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

impl TryFrom<PExperimentStatus> for DExperimentStatus {
    type Error = DomainError;

    fn try_from(PExperimentStatus { ty }: PExperimentStatus) -> DomainResult<Self> {
        use crate::proto::core::p_experiment_status::*;

        Ok(match convert!(ty?) {
            Ty::Idle(PIdle { since }) => {
                DExperimentStatus::Idle {
                    since: convert!(since as DateTime),
                }
            }

            Ty::Running(PRunning { since, last_heartbeat_at, completed_jobs, total_jobs }) => {
                DExperimentStatus::Running {
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

                DExperimentStatus::Completed {
                    since: convert!(since as DateTime),
                    result,
                }
            }

            Ty::Zombie(PZombie { since }) => {
                DExperimentStatus::Zombie {
                    since: convert!(since as DateTime),
                }
            }
        })
    }
}

impl Into<PExperimentStatus> for DExperimentStatus {
    fn into(self) -> PExperimentStatus {
        use crate::proto::core::p_experiment_status::*;

        let ty = match self {
            DExperimentStatus::Idle { since } => {
                Ty::Idle(PIdle {
                    since: since.to_rfc3339(),
                })
            }

            DExperimentStatus::Running { since, last_heartbeat_at, completed_jobs, total_jobs } => {
                Ty::Running(PRunning {
                    since: since.to_rfc3339(),
                    last_heartbeat_at: last_heartbeat_at.to_rfc3339(),
                    completed_jobs,
                    total_jobs,
                })
            }

            DExperimentStatus::Completed { since, result } => {
                Ty::Completed(PCompleted {
                    since: since.to_rfc3339(),
                    success: result.is_ok(),
                    cause: result.err().unwrap_or_default(),
                })
            }

            DExperimentStatus::Zombie { since } => {
                Ty::Zombie(PZombie {
                    since: since.to_rfc3339(),
                })
            }
        };

        PExperimentStatus { ty: Some(ty) }
    }
}