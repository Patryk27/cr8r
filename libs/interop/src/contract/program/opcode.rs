use std::convert::TryFrom;

use crate::{Error, parse, Result};
use crate::contract::CAttachmentId;
use crate::protocol::core::PProgramOpcode;

#[derive(Clone, Debug)]
pub enum CProgramOpcode {
    LogSystemMsg {
        msg: String,
    },

    LogUserMsg {
        msg: String,
    },

    Exec {
        cmd: String,
    },

    PatchCrate {
        name: String,
        attachment_id: CAttachmentId,
    },
}

impl CProgramOpcode {
    pub fn log_system_msg(msg: impl Into<String>) -> Self {
        CProgramOpcode::LogSystemMsg {
            msg: msg.into(),
        }
    }

    pub fn log_user_msg(msg: impl Into<String>) -> Self {
        CProgramOpcode::LogUserMsg {
            msg: msg.into(),
        }
    }

    pub fn exec(cmd: impl Into<String>) -> Self {
        CProgramOpcode::Exec {
            cmd: cmd.into(),
        }
    }

    pub fn patch_crate(name: impl Into<String>, attachment_id: impl Into<CAttachmentId>) -> Self {
        CProgramOpcode::PatchCrate {
            name: name.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

impl TryFrom<PProgramOpcode> for CProgramOpcode {
    type Error = Error;

    fn try_from(PProgramOpcode { op }: PProgramOpcode) -> Result<Self> {
        use crate::protocol::core::p_program_opcode::*;

        Ok(match parse!(op?) {
            Op::LogSystemMsg(PLogSystemMsg { msg }) => {
                CProgramOpcode::LogSystemMsg { msg }
            }

            Op::LogUserMsg(PLogUserMsg { msg }) => {
                CProgramOpcode::LogUserMsg { msg }
            }

            Op::Exec(PExec { cmd }) => {
                CProgramOpcode::Exec { cmd }
            }
        })
    }
}

impl Into<PProgramOpcode> for CProgramOpcode {
    fn into(self) -> PProgramOpcode {
        use crate::protocol::core::p_program_opcode::*;

        let op = match self {
            CProgramOpcode::LogSystemMsg { msg } => {
                Op::LogSystemMsg(PLogSystemMsg { msg })
            }

            CProgramOpcode::LogUserMsg { msg } => {
                Op::LogUserMsg(PLogUserMsg { msg })
            }

            CProgramOpcode::Exec { cmd } => {
                Op::Exec(PExec { cmd })
            }

            CProgramOpcode::PatchCrate { name, attachment_id } => {
                unimplemented!()
            }
        };

        PProgramOpcode {
            op: Some(op),
        }
    }
}