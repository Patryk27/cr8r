use std::convert::TryFrom;

use crate::{convert, Error, Result};
use crate::contract::CAttachmentId;
use crate::protocol::core::PExperimentDefinition;

#[derive(Clone, Debug)]
pub enum CExperimentDefinition {
    OverrideToolchain {
        toolchain: String,
    },

    OverrideCrate {
        name: String,
        version: String,
    },

    PatchCrate {
        name: String,
        attachment_id: CAttachmentId,
    },
}

impl TryFrom<PExperimentDefinition> for CExperimentDefinition {
    type Error = Error;

    fn try_from(PExperimentDefinition { ty }: PExperimentDefinition) -> Result<Self> {
        use crate::protocol::core::p_experiment_definition::*;

        Ok(match convert!(ty?) {
            Ty::OverrideToolchain(POverrideToolchain { toolchain }) => {
                CExperimentDefinition::OverrideToolchain { toolchain }
            }

            Ty::OverrideCrate(POverrideCrate { name, version }) => {
                CExperimentDefinition::OverrideCrate { name, version }
            }

            Ty::PatchCrate(PPatchCrate { name, attachment_id }) => {
                CExperimentDefinition::PatchCrate { name, attachment_id: attachment_id.into() }
            }
        })
    }
}

impl Into<PExperimentDefinition> for CExperimentDefinition {
    fn into(self) -> PExperimentDefinition {
        use crate::protocol::core::p_experiment_definition::*;

        let ty = match self {
            CExperimentDefinition::OverrideToolchain { toolchain } => {
                Ty::OverrideToolchain(POverrideToolchain { toolchain })
            }

            CExperimentDefinition::OverrideCrate { name, version } => {
                Ty::OverrideCrate(POverrideCrate { name, version })
            }

            CExperimentDefinition::PatchCrate { name, attachment_id } => {
                Ty::PatchCrate(PPatchCrate { name, attachment_id: attachment_id.into() })
            }
        };

        PExperimentDefinition { ty: Some(ty) }
    }
}