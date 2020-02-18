use std::convert::TryFrom;
use std::result;

use chrono::{DateTime, Utc};

use crate::conv;
use crate::models::{ModelError, ModelResult};
use crate::proto::models::PExperimentStatus;

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

    Stopped {
        since: DateTime<Utc>,
    },
}

impl TryFrom<PExperimentStatus> for DExperimentStatus {
    type Error = ModelError;

    fn try_from(PExperimentStatus { ty }: PExperimentStatus) -> ModelResult<Self> {
        use crate::proto::models::p_experiment_status::*;

        Ok(match conv!(ty?) {
            Ty::Idle(PIdle { since }) => {
                DExperimentStatus::Idle {
                    since: conv!(since as DateTime),
                }
            }

            Ty::Running(PRunning { since, last_heartbeat_at, completed_jobs, total_jobs }) => {
                DExperimentStatus::Running {
                    since: conv!(since as DateTime),
                    last_heartbeat_at: conv!(last_heartbeat_at as DateTime),
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
                    since: conv!(since as DateTime),
                    result,
                }
            }

            Ty::Stopped(PStopped { since }) => {
                DExperimentStatus::Stopped {
                    since: conv!(since as DateTime),
                }
            }
        })
    }
}

impl Into<PExperimentStatus> for DExperimentStatus {
    fn into(self) -> PExperimentStatus {
        use crate::proto::models::p_experiment_status::*;

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

            DExperimentStatus::Stopped { since } => {
                Ty::Stopped(PStopped {
                    since: since.to_rfc3339(),
                })
            }
        };

        PExperimentStatus { ty: Some(ty) }
    }
}