use std::convert::TryFrom;

use crate::{convert, Error, Result};
use crate::contract::CAttachmentId;
use crate::protocol::core::PJobOpcode;

#[derive(Clone, Debug)]
pub enum CJobOpcode {
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

impl CJobOpcode {
    pub fn log_system_msg(msg: impl Into<String>) -> Self {
        CJobOpcode::LogSystemMsg {
            msg: msg.into(),
        }
    }

    pub fn log_user_msg(msg: impl Into<String>) -> Self {
        CJobOpcode::LogUserMsg {
            msg: msg.into(),
        }
    }

    pub fn exec(cmd: impl Into<String>) -> Self {
        CJobOpcode::Exec {
            cmd: cmd.into(),
        }
    }

    pub fn patch_crate(name: impl Into<String>, attachment_id: impl Into<CAttachmentId>) -> Self {
        CJobOpcode::PatchCrate {
            name: name.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

impl TryFrom<PJobOpcode> for CJobOpcode {
    type Error = Error;

    fn try_from(PJobOpcode { ty }: PJobOpcode) -> Result<Self> {
        use crate::protocol::core::p_job_opcode::*;

        Ok(match convert!(ty?) {
            Ty::LogSystemMsg(PLogSystemMsg { msg }) => {
                CJobOpcode::LogSystemMsg { msg }
            }

            Ty::LogUserMsg(PLogUserMsg { msg }) => {
                CJobOpcode::LogUserMsg { msg }
            }

            Ty::Exec(PExec { cmd }) => {
                CJobOpcode::Exec { cmd }
            }
        })
    }
}

impl Into<PJobOpcode> for CJobOpcode {
    fn into(self) -> PJobOpcode {
        use crate::protocol::core::p_job_opcode::*;

        let ty = match self {
            CJobOpcode::LogSystemMsg { msg } => {
                Ty::LogSystemMsg(PLogSystemMsg { msg })
            }

            CJobOpcode::LogUserMsg { msg } => {
                Ty::LogUserMsg(PLogUserMsg { msg })
            }

            CJobOpcode::Exec { cmd } => {
                Ty::Exec(PExec { cmd })
            }

            CJobOpcode::PatchCrate { .. } => {
                unimplemented!()
            }
        };

        PJobOpcode { ty: Some(ty) }
    }
}