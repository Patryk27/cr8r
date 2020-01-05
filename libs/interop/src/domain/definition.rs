use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DAttachmentId, DomainError, DomainResult};
use crate::proto::core::PDefinition;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DDefinition {
    pub overridden_toolchain: Option<definition_inner::DOverriddenToolchain>,
    pub overridden_packages: Vec<definition_inner::DOverriddenPackage>,
    pub patched_packages: Vec<definition_inner::DPatchedPackage>,
}

pub mod definition_inner {
    use crate::domain::DAttachmentId;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct DOverriddenToolchain {
        pub toolchain: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct DOverriddenPackage {
        pub name: String,
        pub version: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct DPatchedPackage {
        pub name: String,
        pub attachment_id: DAttachmentId,
    }
}

impl TryFrom<PDefinition> for DDefinition {
    type Error = DomainError;

    fn try_from(value: PDefinition) -> DomainResult<Self> {
        unimplemented!()
    }
}

impl Into<PDefinition> for DDefinition {
    fn into(self) -> PDefinition {
        unimplemented!()
    }
}