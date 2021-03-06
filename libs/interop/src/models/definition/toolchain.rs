use std::convert::TryFrom;

use crate::models::{ModelError, ModelResult};
use crate::proto::models::p_definition::PToolchainDef;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DToolchainDef {
    pub toolchain: String,
}

impl TryFrom<PToolchainDef> for DToolchainDef {
    type Error = ModelError;

    fn try_from(PToolchainDef { toolchain }: PToolchainDef) -> ModelResult<Self> {
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