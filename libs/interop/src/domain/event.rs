use std::convert::TryFrom;
use std::result;

use chrono::{DateTime, Utc};

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::proto::core::PEvent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DEvent {
    pub at: DateTime<Utc>,
    pub ty: DEventType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DEventType {
    Ping,

    SystemMsg {
        msg: String,
    },

    UserMsg {
        msg: String,
    },

    ProcessOutput {
        line: String,
    },

    ExperimentStarted,

    ExperimentCompleted,

    JobStarted {
        id: usize,
    },

    JobCompleted {
        id: usize,
        result: result::Result<(), String>,
    },
}

impl TryFrom<PEvent> for DEvent {
    type Error = DomainError;

    fn try_from(PEvent { at, ty }: PEvent) -> DomainResult<Self> {
        use crate::proto::core::p_event::*;

        let ty = match convert!(ty?) {
            Ty::Ping(_) => {
                DEventType::Ping
            }

            Ty::SystemMsg(PSystemMsg { msg }) => {
                DEventType::SystemMsg { msg }
            }

            Ty::UserMsg(PUserMsg { msg }) => {
                DEventType::UserMsg { msg }
            }

            Ty::ProcessOutput(PProcessOutput { line }) => {
                DEventType::ProcessOutput { line }
            }

            Ty::ExperimentStarted(_) => {
                DEventType::ExperimentStarted
            }

            Ty::ExperimentCompleted(_) => {
                DEventType::ExperimentCompleted
            }

            Ty::JobStarted(PJobStarted { id }) => {
                DEventType::JobStarted { id: id as _ }
            }

            Ty::JobCompleted(PJobCompleted { id, failure_cause }) => {
                let result = if failure_cause.is_empty() {
                    Ok(())
                } else {
                    Err(failure_cause)
                };

                DEventType::JobCompleted { id: id as _, result }
            }
        };

        Ok(Self {
            at: convert!(at as DateTime),
            ty,
        })
    }
}

impl Into<PEvent> for DEvent {
    fn into(self) -> PEvent {
        use crate::proto::core::p_event::*;

        let ty = match self.ty {
            DEventType::Ping => {
                Ty::Ping(PPing {})
            }

            DEventType::SystemMsg { msg } => {
                Ty::SystemMsg(PSystemMsg { msg })
            }

            DEventType::UserMsg { msg } => {
                Ty::UserMsg(PUserMsg { msg })
            }

            DEventType::ProcessOutput { line } => {
                Ty::ProcessOutput(PProcessOutput { line })
            }

            DEventType::ExperimentStarted => {
                Ty::ExperimentStarted(PExperimentStarted {})
            }

            DEventType::ExperimentCompleted => {
                Ty::ExperimentCompleted(PExperimentCompleted {})
            }

            DEventType::JobStarted { id } => {
                Ty::JobStarted(PJobStarted { id: id as _ })
            }

            DEventType::JobCompleted { id, result } => {
                Ty::JobCompleted(PJobCompleted {
                    id: id as _,
                    failure_cause: result.err().unwrap_or_default(),
                })
            }
        };

        PEvent {
            at: self.at.to_rfc3339(),
            ty: Some(ty),
        }
    }
}