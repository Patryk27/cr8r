use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DAttachmentId, DomainError, DomainResult};
use crate::proto::core::PDefinition;

#[derive(Clone, Debug)]
pub struct DDefinition {
    pub overridden_toolchain: definition::DOverriddenToolchain,
    pub overridden_dependencies: Vec<definition::DOverriddenDependency>,
    pub patched_dependencies: Vec<definition::DPatchedDependency>,
}

pub mod definition {
    use crate::domain::DAttachmentId;

    #[derive(Clone, Debug)]
    pub struct DOverriddenToolchain {
        pub toolchain: String,
    }

    #[derive(Clone, Debug)]
    pub struct DOverriddenDependency {
        pub name: String,
        pub version: String,
    }

    #[derive(Clone, Debug)]
    pub struct DPatchedDependency {
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