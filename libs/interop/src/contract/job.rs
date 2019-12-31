use std::convert::TryFrom;

use crate::{convert, Error, Result};
use crate::protocol::core::PJob;

pub use self::opcode::*;

mod opcode;

#[derive(Clone, Debug)]
pub struct CJob {
    pub name: String,
    pub toolchain: String,
    pub opcodes: Vec<CJobOpcode>,
}

impl TryFrom<PJob> for CJob {
    type Error = Error;

    fn try_from(PJob { name, toolchain, opcodes }: PJob) -> Result<Self> {
        Ok(Self {
            name,
            toolchain,
            opcodes: convert!(opcodes as [_?]),
        })
    }
}

impl Into<PJob> for CJob {
    fn into(self) -> PJob {
        let Self { name, toolchain, opcodes } = self;

        PJob {
            name,
            toolchain,
            opcodes: convert!(opcodes as [_]),
        }
    }
}