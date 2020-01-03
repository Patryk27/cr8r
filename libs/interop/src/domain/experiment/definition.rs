use std::convert::TryFrom;

use crate::{convert, Error, Result};
use crate::domain::DAttachmentId;
use crate::proto::core::PExperimentDefinition;

#[derive(Clone, Debug)]
pub enum DExperimentDefinition {
    OverrideToolchain {
        toolchain: String,
    },

    OverrideCrate {
        name: String,
        version: String,
    },

    PatchCrate {
        name: String,
        attachment_id: DAttachmentId,
    },
}

impl TryFrom<PExperimentDefinition> for DExperimentDefinition {
    type Error = Error;

    fn try_from(PExperimentDefinition { ty }: PExperimentDefinition) -> Result<Self> {
        use crate::proto::core::p_experiment_definition::*;

        Ok(match convert!(ty?) {
            Ty::OverrideToolchain(POverrideToolchain { toolchain }) => {
                DExperimentDefinition::OverrideToolchain { toolchain }
            }

            Ty::OverrideCrate(POverrideCrate { name, version }) => {
                DExperimentDefinition::OverrideCrate { name, version }
            }

            Ty::PatchCrate(PPatchCrate { name, attachment_id }) => {
                DExperimentDefinition::PatchCrate { name, attachment_id: attachment_id.into() }
            }
        })
    }
}

impl Into<PExperimentDefinition> for DExperimentDefinition {
    fn into(self) -> PExperimentDefinition {
        use crate::proto::core::p_experiment_definition::*;

        let ty = match self {
            DExperimentDefinition::OverrideToolchain { toolchain } => {
                Ty::OverrideToolchain(POverrideToolchain { toolchain })
            }

            DExperimentDefinition::OverrideCrate { name, version } => {
                Ty::OverrideCrate(POverrideCrate { name, version })
            }

            DExperimentDefinition::PatchCrate { name, attachment_id } => {
                Ty::PatchCrate(PPatchCrate { name, attachment_id: attachment_id.into() })
            }
        };

        PExperimentDefinition { ty: Some(ty) }
    }
}