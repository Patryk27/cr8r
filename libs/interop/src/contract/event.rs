use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::{Error, parse, Result};
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

    ExperimentSucceeded,

    ExperimentFailed {
        cause: String,
    },

    OpcodeSucceeded {
        id: u32,
    },

    OpcodeFailed {
        id: u32,
        cause: String,
    },
}

impl TryFrom<PEvent> for CEvent {
    type Error = Error;

    fn try_from(PEvent { created_at, op }: PEvent) -> Result<Self> {
        use crate::protocol::core::p_event::*;

        let ty = match parse!(op?) {
            Op::Ping(_) => {
                CEventType::Ping
            }

            Op::SystemMsg(PSystemMsg { msg }) => {
                CEventType::SystemMsg { msg }
            }

            Op::UserMsg(PUserMsg { msg }) => {
                CEventType::UserMsg { msg }
            }

            Op::ProcessOutput(PProcessOutput { line }) => {
                CEventType::ProcessOutput { line }
            }

            Op::ExperimentStarted(_) => {
                CEventType::ExperimentStarted
            }

            Op::ExperimentSucceeded(_) => {
                CEventType::ExperimentSucceeded
            }

            Op::ExperimentFailed(PExperimentFailed { cause }) => {
                CEventType::ExperimentFailed { cause }
            }

            Op::OpcodeSucceeded(POpcodeSucceeded { id }) => {
                CEventType::OpcodeSucceeded { id }
            }

            Op::OpcodeFailed(POpcodeFailed { id, cause }) => {
                CEventType::OpcodeFailed { id, cause }
            }
        };

        Ok(Self {
            at: parse!(created_at as DateTime),
            ty,
        })
    }
}

impl Into<PEvent> for CEvent {
    fn into(self) -> PEvent {
        use crate::protocol::core::p_event::*;

        let op = match self.ty {
            CEventType::Ping => {
                Op::Ping(PPing {})
            }

            CEventType::SystemMsg { msg } => {
                Op::SystemMsg(PSystemMsg { msg })
            }

            CEventType::UserMsg { msg } => {
                Op::UserMsg(PUserMsg { msg })
            }

            CEventType::ProcessOutput { line } => {
                Op::ProcessOutput(PProcessOutput { line })
            }

            CEventType::ExperimentStarted => {
                Op::ExperimentStarted(PExperimentStarted {})
            }

            CEventType::ExperimentSucceeded => {
                Op::ExperimentSucceeded(PExperimentSucceeded {})
            }

            CEventType::ExperimentFailed { cause } => {
                Op::ExperimentFailed(PExperimentFailed { cause })
            }

            CEventType::OpcodeSucceeded { id } => {
                Op::OpcodeSucceeded(POpcodeSucceeded { id })
            }

            CEventType::OpcodeFailed { id, cause } => {
                Op::OpcodeFailed(POpcodeFailed { id, cause })
            }
        };

        PEvent {
            created_at: self.at.to_rfc3339(),
            op: Some(op),
        }
    }
}