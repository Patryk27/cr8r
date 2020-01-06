use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DAttachmentId, DomainError, DomainResult};
use crate::proto::core::p_definition::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DPackage {
    Overridden {
        version: String,
    },

    Patched {
        attachment_id: DAttachmentId,
    },
}

impl TryFrom<PPackage> for DPackage {
    type Error = DomainError;

    fn try_from(PPackage { ty }: PPackage) -> DomainResult<Self> {
        use p_package::*;

        Ok(match convert!(ty?) {
            Ty::Overridden(POverridden { version }) => {
                DPackage::Overridden { version }
            }

            Ty::Patched(PPatched { attachment_id }) => {
                DPackage::Patched {
                    attachment_id: convert!(attachment_id as _),
                }
            }
        })
    }
}

impl Into<PPackage> for DPackage {
    fn into(self) -> PPackage {
        use p_package::*;

        let ty = match self {
            DPackage::Overridden { version } => {
                Ty::Overridden(POverridden { version })
            }

            DPackage::Patched { attachment_id } => {
                Ty::Patched(PPatched {
                    attachment_id: convert!(attachment_id as _),
                })
            }
        };

        PPackage { ty: Some(ty) }
    }
}