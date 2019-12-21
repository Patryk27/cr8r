use std::convert::TryFrom;

use crate::{Error, Result};
use crate::protocol::core::PEvent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CEvent {
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

    OpcodeSucceeded,

    OpcodeFailed {
        id: u32,
        cause: String,
    },
}

impl TryFrom<PEvent> for CEvent {
    type Error = Error;

    fn try_from(event: PEvent) -> Result<Self> {
        unimplemented!()
    }
}

impl Into<PEvent> for CEvent {
    fn into(self) -> PEvent {
        unimplemented!()
    }
}