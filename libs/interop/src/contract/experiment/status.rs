use std::convert::TryFrom;
use std::result;

use chrono::{DateTime, Utc};
use snafu::ResultExt;

use crate::{Error, error, Result};
use crate::protocol::core::p_experiment::PStatus;

#[derive(Clone, Debug)]
pub enum CExperimentStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat_at: DateTime<Utc>,
        completed_steps: u32,
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

    fn try_from(status: PStatus) -> Result<Self> {
        use crate::protocol::core::p_experiment::p_status::*;

        let op = status.op.ok_or_else(|| Error::Missing { name: "op" })?;

        Ok(match op {
            Op::Idle(PIdle { since }) => {
                CExperimentStatus::Idle {
                    since: DateTime::parse_from_rfc3339(&since)
                        .context(error::InvalidDateTime { name: "since" })?
                        .with_timezone(&Utc),
                }
            }

            _ => {
                unimplemented!()
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

            CExperimentStatus::Running { since, last_heartbeat_at, completed_steps } => {
                Op::Running(PRunning {
                    since: since.to_rfc3339(),
                    last_heartbeat_at: last_heartbeat_at.to_rfc3339(),
                    completed_steps,
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