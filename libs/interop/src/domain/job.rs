use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::proto::core::PJob;

pub use self::opcode::*;

mod opcode;

#[derive(Clone, Debug)]
pub struct DJob {
    pub name: String,
    pub toolchain: String,
    pub opcodes: Vec<DJobOpcode>,
}

impl TryFrom<PJob> for DJob {
    type Error = DomainError;

    fn try_from(PJob { name, toolchain, opcodes }: PJob) -> DomainResult<Self> {
        Ok(Self {
            name,
            toolchain,
            opcodes: convert!(opcodes as [_?]),
        })
    }
}

impl Into<PJob> for DJob {
    fn into(self) -> PJob {
        let Self { name, toolchain, opcodes } = self;

        PJob {
            name,
            toolchain,
            opcodes: convert!(opcodes as [_]),
        }
    }
}