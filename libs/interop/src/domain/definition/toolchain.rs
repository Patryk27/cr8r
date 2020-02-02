use std::convert::TryFrom;

use crate::domain::{DomainError, DomainResult};
use crate::proto::core::p_definition::PToolchainDef;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DToolchainDef {
    pub toolchain: String,
}

impl TryFrom<PToolchainDef> for DToolchainDef {
    type Error = DomainError;

    fn try_from(PToolchainDef { toolchain }: PToolchainDef) -> DomainResult<Self> {
        Ok(Self { toolchain })
    }
}

impl Into<PToolchainDef> for DToolchainDef {
    fn into(self) -> PToolchainDef {
        PToolchainDef {
            toolchain: self.toolchain,
        }
    }
}