use std::convert::TryFrom;
use std::result;

use chrono::{DateTime, Utc};

use crate::{convert, Error, Result};
use crate::protocol::core::PEvent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CEvent {
    pub at: DateTime<Utc>,
    pub ty: CEventType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CEventType {
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

impl TryFrom<PEvent> for CEvent {
    type Error = Error;

    fn try_from(PEvent { at, ty }: PEvent) -> Result<Self> {
        use crate::protocol::core::p_event::*;

        let ty = match convert!(ty?) {
            Ty::Ping(_) => {
                CEventType::Ping
            }

            Ty::SystemMsg(PSystemMsg { msg }) => {
                CEventType::SystemMsg { msg }
            }

            Ty::UserMsg(PUserMsg { msg }) => {
                CEventType::UserMsg { msg }
            }

            Ty::ProcessOutput(PProcessOutput { line }) => {
                CEventType::ProcessOutput { line }
            }

            Ty::ExperimentStarted(_) => {
                CEventType::ExperimentStarted
            }

            Ty::ExperimentCompleted(_) => {
                CEventType::ExperimentCompleted
            }

            Ty::JobStarted(PJobStarted { id }) => {
                CEventType::JobStarted { id: id as _ }
            }

            Ty::JobCompleted(PJobCompleted { id, failure_cause }) => {
                let result = if failure_cause.is_empty() {
                    Ok(())
                } else {
                    Err(failure_cause)
                };

                CEventType::JobCompleted { id: id as _, result }
            }
        };

        Ok(Self {
            at: convert!(at as DateTime),
            ty,
        })
    }
}

impl Into<PEvent> for CEvent {
    fn into(self) -> PEvent {
        use crate::protocol::core::p_event::*;

        let ty = match self.ty {
            CEventType::Ping => {
                Ty::Ping(PPing {})
            }

            CEventType::SystemMsg { msg } => {
                Ty::SystemMsg(PSystemMsg { msg })
            }

            CEventType::UserMsg { msg } => {
                Ty::UserMsg(PUserMsg { msg })
            }

            CEventType::ProcessOutput { line } => {
                Ty::ProcessOutput(PProcessOutput { line })
            }

            CEventType::ExperimentStarted => {
                Ty::ExperimentStarted(PExperimentStarted {})
            }

            CEventType::ExperimentCompleted => {
                Ty::ExperimentCompleted(PExperimentCompleted {})
            }

            CEventType::JobStarted { id } => {
                Ty::JobStarted(PJobStarted { id: id as _ })
            }

            CEventType::JobCompleted { id, result } => {
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