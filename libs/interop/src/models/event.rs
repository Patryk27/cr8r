use std::convert::TryFrom;
use std::result;

use chrono::{DateTime, Utc};

use crate::conv;
use crate::models::{ModelError, ModelResult};
use crate::models::job::DJobId;
use crate::proto::models::PEvent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DEvent {
    pub at: DateTime<Utc>,
    pub ty: DEventType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DEventType {
    Heartbeat,

    SystemMsg {
        msg: String,
    },

    ProcessMsg {
        msg: String,
    },

    ExperimentStarted,

    ExperimentCompleted,

    JobStarted {
        id: DJobId,
    },

    JobCompleted {
        id: DJobId,
        result: result::Result<(), String>,
    },
}

impl TryFrom<PEvent> for DEvent {
    type Error = ModelError;

    fn try_from(PEvent { at, ty }: PEvent) -> ModelResult<Self> {
        use crate::proto::models::p_event::*;

        let ty = match conv!(ty?) {
            Ty::Heartbeat(_) => {
                DEventType::Heartbeat
            }

            Ty::SystemMsg(PSystemMsg { msg }) => {
                DEventType::SystemMsg { msg }
            }

            Ty::ProcessMsg(PProcessMsg { msg }) => {
                DEventType::ProcessMsg { msg }
            }

            Ty::ExperimentStarted(_) => {
                DEventType::ExperimentStarted
            }

            Ty::ExperimentCompleted(_) => {
                DEventType::ExperimentCompleted
            }

            Ty::JobStarted(PJobStarted { id }) => {
                DEventType::JobStarted {
                    id: conv!(id as _),
                }
            }

            Ty::JobCompleted(PJobCompleted { id, failure_cause }) => {
                let result = if failure_cause.is_empty() {
                    Ok(())
                } else {
                    Err(failure_cause)
                };

                DEventType::JobCompleted {
                    id: conv!(id as _),
                    result,
                }
            }
        };

        Ok(Self {
            at: conv!(at as DateTime),
            ty,
        })
    }
}

impl Into<PEvent> for DEvent {
    fn into(self) -> PEvent {
        use crate::proto::models::p_event::*;

        let ty = match self.ty {
            DEventType::Heartbeat => {
                Ty::Heartbeat(PHeartbeat {})
            }

            DEventType::SystemMsg { msg } => {
                Ty::SystemMsg(PSystemMsg { msg })
            }

            DEventType::ProcessMsg { msg } => {
                Ty::ProcessMsg(PProcessMsg { msg })
            }

            DEventType::ExperimentStarted => {
                Ty::ExperimentStarted(PExperimentStarted {})
            }

            DEventType::ExperimentCompleted => {
                Ty::ExperimentCompleted(PExperimentCompleted {})
            }

            DEventType::JobStarted { id } => {
                Ty::JobStarted(PJobStarted {
                    id: conv!(id as _),
                })
            }

            DEventType::JobCompleted { id, result } => {
                Ty::JobCompleted(PJobCompleted {
                    id: conv!(id as _),
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