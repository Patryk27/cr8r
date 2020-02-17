use std::convert::TryFrom;

use crate::conv;
use crate::domain::{DomainError, DomainResult};
use crate::proto::models::PDefinition;

pub use self::{
    dependency::*,
    toolchain::*,
};

mod dependency;
mod toolchain;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DDefinition {
    pub toolchain: Option<DToolchainDef>,
    pub dependencies: Vec<DDependencyDef>,
}

impl TryFrom<PDefinition> for DDefinition {
    type Error = DomainError;

    fn try_from(PDefinition { toolchain, dependencies }: PDefinition) -> DomainResult<Self> {
        let toolchain = toolchain
            .map(|toolchain| Ok(conv!(toolchain as _?)))
            .transpose()?;

        let dependencies = conv!(dependencies as [_?]);

        Ok(Self { toolchain, dependencies })
    }
}

impl Into<PDefinition> for DDefinition {
    fn into(self) -> PDefinition {
        let Self { toolchain, dependencies } = self;

        let toolchain = toolchain.map(|toolchain| conv!(toolchain as _));
        let dependencies = conv!(dependencies as [_]);

        PDefinition { toolchain, dependencies }
    }
}