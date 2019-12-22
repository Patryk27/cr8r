use std::convert::TryFrom;

use crate::{Error, parse, Result};
use crate::protocol::core::PProgram;

pub use self::opcode::*;

mod opcode;

#[derive(Clone, Debug)]
pub struct CProgram {
    pub system: String,
    pub toolchain: String,
    pub opcodes: Vec<CProgramOpcode>,
}

impl TryFrom<PProgram> for CProgram {
    type Error = Error;

    fn try_from(PProgram { system, toolchain, opcodes }: PProgram) -> Result<Self> {
        Ok(Self {
            system,
            toolchain,
            opcodes: parse!(opcodes as [_?]),
        })
    }
}

impl Into<PProgram> for CProgram {
    fn into(self) -> PProgram {
        let opcodes = self.opcodes;

        PProgram {
            system: self.system,
            toolchain: self.toolchain,
            opcodes: parse!(opcodes as [_]),
        }
    }
}