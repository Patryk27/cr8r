use std::convert::TryFrom;
use std::result;

use chrono::{DateTime, Utc};

use crate::{Error, parse, Result};
use crate::protocol::core::p_experiment::PStatus;

#[derive(Clone, Debug)]
pub enum CExperimentStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat_at: DateTime<Utc>,
        completed_ops: u32,
    },

    Completed {
        since: DateTime<Utc>,
        result: result::Result<(), String>,
    },

    Zombie {
        since: DateTime<Utc>,
    },
}

impl TryFrom<PStatus> for CExperimentStatus {
    type Error = Error;

    fn try_from(PStatus { op }: PStatus) -> Result<Self> {
        use crate::protocol::core::p_experiment::p_status::*;

        Ok(match parse!(op?) {
            Op::Idle(PIdle { since }) => {
                CExperimentStatus::Idle {
                    since: parse!(since as DateTime),
                }
            }

            Op::Running(PRunning { since, last_heartbeat_at, completed_ops }) => {
                CExperimentStatus::Running {
                    since: parse!(since as DateTime),
                    last_heartbeat_at: parse!(last_heartbeat_at as DateTime),
                    completed_ops,
                }
            }

            Op::Completed(PCompleted { since, success, cause }) => {
                let result = if success {
                    Ok(())
                } else {
                    Err(cause)
                };

                CExperimentStatus::Completed {
                    since: parse!(since as DateTime),
                    result,
                }
            }

            Op::Zombie(PZombie { since }) => {
                CExperimentStatus::Zombie {
                    since: parse!(since as DateTime),
                }
            }
        })
    }
}

impl Into<PStatus> for CExperimentStatus {
    fn into(self) -> PStatus {
        use crate::protocol::core::p_experiment::p_status::*;

        let op = match self {
            CExperimentStatus::Idle { since } => {
                Op::Idle(PIdle {
                    since: since.to_rfc3339(),
                })
            }

            CExperimentStatus::Running { since, last_heartbeat_at, completed_ops } => {
                Op::Running(PRunning {
                    since: since.to_rfc3339(),
                    last_heartbeat_at: last_heartbeat_at.to_rfc3339(),
                    completed_ops,
                })
            }

            CExperimentStatus::Completed { since, result } => {
                Op::Completed(PCompleted {
                    since: since.to_rfc3339(),
                    success: result.is_ok(),
                    cause: result.err().unwrap_or_default(),
                })
            }

            CExperimentStatus::Zombie { since } => {
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