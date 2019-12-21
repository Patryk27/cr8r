use crate::contract::CAttachmentId;
use crate::protocol::core::PExperimentStep;

#[derive(Clone, Debug)]
pub enum CProgramStep {
    Exec {
        cmd: String,
    },

    LogSystemMsg {
        msg: String,
    },

    LogUserMsg {
        msg: String,
    },

    PatchCrate {
        name: String,
        attachment_id: CAttachmentId,
    },
}

impl Into<PExperimentStep> for CProgramStep {
    fn into(self) -> PExperimentStep {
        let op = match self {
            CProgramStep::Exec { cmd } => {
                unimplemented!()
            }

            CProgramStep::LogSystemMsg { msg } => {
                unimplemented!()
            }

            CProgramStep::LogUserMsg { msg } => {
                unimplemented!()
            }

            CProgramStep::PatchCrate { name, attachment_id } => {
                unimplemented!()
            }
        };

        PExperimentStep {
            op: Some(op),
        }
    }
}