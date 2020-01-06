use std::convert::TryFrom;

use crate::domain::{DomainError, DomainResult};
use crate::proto::core::p_definition::PToolchain;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DToolchain {
    pub version: String,
}

impl TryFrom<PToolchain> for DToolchain {
    type Error = DomainError;

    fn try_from(PToolchain { version }: PToolchain) -> DomainResult<Self> {
        Ok(Self { version })
    }
}

impl Into<PToolchain> for DToolchain {
    fn into(self) -> PToolchain {
        PToolchain {
            version: self.version,
        }
    }
}