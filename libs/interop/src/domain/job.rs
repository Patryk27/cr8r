use std::convert::TryFrom;

use crate::conv;
use crate::domain::{DomainError, DomainResult};
use crate::proto::models::PJob;

pub use self::opcode::DJobOpcode;

pub mod opcode;

#[derive(Clone, Debug)]
pub struct DJob {
    pub name: String,
    pub opcodes: Vec<DJobOpcode>,
}

impl TryFrom<PJob> for DJob {
    type Error = DomainError;

    fn try_from(PJob { name, opcodes }: PJob) -> DomainResult<Self> {
        Ok(Self {
            name,
            opcodes: conv!(opcodes as [_?]),
        })
    }
}

impl Into<PJob> for DJob {
    fn into(self) -> PJob {
        let Self { name, opcodes } = self;

        PJob {
            name,
            opcodes: conv!(opcodes as [_]),
        }
    }
}