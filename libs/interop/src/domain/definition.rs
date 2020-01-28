use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::proto::core::PDefinition;

pub mod definition_inner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DDefinition {
    pub toolchain: Option<definition_inner::DToolchainDef>,
    pub dependencies: Vec<definition_inner::DDependencyDef>,
}

impl TryFrom<PDefinition> for DDefinition {
    type Error = DomainError;

    fn try_from(PDefinition { toolchain, dependencies }: PDefinition) -> DomainResult<Self> {
        let toolchain = toolchain
            .map(|toolchain| Ok(convert!(toolchain as _?)))
            .transpose()?;

        let dependencies = convert!(dependencies as [_?]);

        Ok(Self { toolchain, dependencies })
    }
}

impl Into<PDefinition> for DDefinition {
    fn into(self) -> PDefinition {
        let Self { toolchain, dependencies } = self;

        let toolchain = toolchain.map(|toolchain| convert!(toolchain as _));
        let dependencies = convert!(dependencies as [_]);

        PDefinition { toolchain, dependencies }
    }
}