use core::result;
use std::convert::TryFrom;

use crate::conv;
use crate::models::{ModelError, ModelResult};
use crate::proto::models::PJobStatus;

#[derive(Clone, Debug)]
pub enum DJobStatus {
    Pending,

    Running,

    Completed {
        result: result::Result<(), String>,
    },
}

impl TryFrom<PJobStatus> for DJobStatus {
    type Error = ModelError;

    fn try_from(PJobStatus { ty }: PJobStatus) -> ModelResult<Self> {
        use crate::proto::models::p_job_status::*;

        Ok(match conv!(ty?) {
            Ty::Pending(PPending {}) => {
                DJobStatus::Pending
            }

            Ty::Running(PRunning { .. }) => {
                DJobStatus::Running
            }

            Ty::Completed(PCompleted { failure_cause }) => {
                let result = if failure_cause.is_empty() {
                    Ok(())
                } else {
                    Err(failure_cause)
                };

                DJobStatus::Completed { result }
            }
        })
    }
}

impl Into<PJobStatus> for DJobStatus {
    fn into(self) -> PJobStatus {
        use crate::proto::models::p_job_status::*;

        let ty = match self {
            DJobStatus::Pending => {
                Ty::Pending(PPending::default())
            }

            DJobStatus::Running => {
                Ty::Running(PRunning::default())
            }

            DJobStatus::Completed { result } => {
                Ty::Completed(PCompleted {
                    failure_cause: result.err().unwrap_or_default(),
                })
            }
        };

        PJobStatus { ty: Some(ty) }
    }
}