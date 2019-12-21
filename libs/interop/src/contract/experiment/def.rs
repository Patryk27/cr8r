use std::convert::TryFrom;

use crate::{Error, Result};
use crate::contract::CAttachmentId;
use crate::protocol::core::PExperimentDef;

#[derive(Clone, Debug)]
pub enum CExperimentDef {
    TryPatchCrate {
        name: String,
        attachment_id: CAttachmentId,
    },

    TryToolchain {
        toolchain: String,
    },
}

impl TryFrom<PExperimentDef> for CExperimentDef {
    type Error = Error;

    fn try_from(def: PExperimentDef) -> Result<Self> {
        use crate::protocol::core::p_experiment_def::{*, Op::*};

        Ok(match def.op {
            Some(TryPatchCrate(PTryPatchCrate { name, attachment_id })) => {
                CExperimentDef::TryPatchCrate {
                    name,
                    attachment_id: attachment_id.into(),
                }
            }

            Some(TryToolchain(PTryToolchain { toolchain })) => {
                CExperimentDef::TryToolchain {
                    toolchain,
                }
            }

            None => {
                return Err(Error::MissingField {
                    name: "op",
                });
            }
        })
    }
}