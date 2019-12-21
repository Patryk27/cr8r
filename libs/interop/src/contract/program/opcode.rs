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

impl Into<PProgramOpcode> for CProgramOpcode {
    fn into(self) -> PProgramOpcode {
        use crate::protocol::core::p_program_opcode::*;

        let op = match self {
            CProgramOpcode::Exec { cmd } => {
                Op::Exec(PExec { cmd })
            }

            CProgramOpcode::LogSystemMsg { msg } => {
                Op::LogSystemMsg(PLogSystemMsg { msg })
            }

            CProgramOpcode::LogUserMsg { msg } => {
                Op::LogUserMsg(PLogUserMsg { msg })
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