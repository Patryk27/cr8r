use std::collections::HashMap;
use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::proto::core::PDefinition;

pub mod definition_inner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DDefinition {
    pub toolchain: Option<definition_inner::DToolchain>,
    pub packages: HashMap<String, definition_inner::DPackage>,
}

impl TryFrom<PDefinition> for DDefinition {
    type Error = DomainError;

    fn try_from(PDefinition { toolchain, packages }: PDefinition) -> DomainResult<Self> {
        let toolchain = toolchain
            .map(|toolchain| Ok(convert!(toolchain as _?)))
            .transpose()?;

        let packages = convert!(packages as { _ => _? });

        Ok(Self { toolchain, packages })
    }
}

impl Into<PDefinition> for DDefinition {
    fn into(self) -> PDefinition {
        let Self { toolchain, packages } = self;

        let toolchain = toolchain.map(|toolchain| convert!(toolchain as _));
        let packages = convert!(packages as { _ => _ });

        PDefinition { toolchain, packages }
    }
}