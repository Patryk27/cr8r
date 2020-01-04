use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DAttachmentId, DomainError, DomainResult};
use crate::proto::core::PJobOpcode;

#[derive(Clone, Debug)]
pub enum DJobOpcode {
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
        attachment_id: DAttachmentId,
    },
}

impl DJobOpcode {
    pub fn log_system_msg(msg: impl Into<String>) -> Self {
        DJobOpcode::LogSystemMsg {
            msg: msg.into(),
        }
    }

    pub fn log_user_msg(msg: impl Into<String>) -> Self {
        DJobOpcode::LogUserMsg {
            msg: msg.into(),
        }
    }

    pub fn exec(cmd: impl Into<String>) -> Self {
        DJobOpcode::Exec {
            cmd: cmd.into(),
        }
    }

    pub fn patch_crate(name: impl Into<String>, attachment_id: impl Into<DAttachmentId>) -> Self {
        DJobOpcode::PatchCrate {
            name: name.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

impl TryFrom<PJobOpcode> for DJobOpcode {
    type Error = DomainError;

    fn try_from(PJobOpcode { ty }: PJobOpcode) -> DomainResult<Self> {
        use crate::proto::core::p_job_opcode::*;

        Ok(match convert!(ty?) {
            Ty::LogSystemMsg(PLogSystemMsg { msg }) => {
                DJobOpcode::LogSystemMsg { msg }
            }

            Ty::LogUserMsg(PLogUserMsg { msg }) => {
                DJobOpcode::LogUserMsg { msg }
            }

            Ty::Exec(PExec { cmd }) => {
                DJobOpcode::Exec { cmd }
            }
        })
    }
}

impl Into<PJobOpcode> for DJobOpcode {
    fn into(self) -> PJobOpcode {
        use crate::proto::core::p_job_opcode::*;

        let ty = match self {
            DJobOpcode::LogSystemMsg { msg } => {
                Ty::LogSystemMsg(PLogSystemMsg { msg })
            }

            DJobOpcode::LogUserMsg { msg } => {
                Ty::LogUserMsg(PLogUserMsg { msg })
            }

            DJobOpcode::Exec { cmd } => {
                Ty::Exec(PExec { cmd })
            }

            DJobOpcode::PatchCrate { .. } => {
                unimplemented!()
            }
        };

        PJobOpcode { ty: Some(ty) }
    }
}