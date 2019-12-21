use std::convert::TryFrom;

use crate::{Error, Result};
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

    fn try_from(program: PProgram) -> Result<Self> {
        unimplemented!()
    }
}

impl Into<PProgram> for CProgram {
    fn into(self) -> PProgram {
        let opcodes = self.opcodes
            .into_iter()
            .map(Into::into)
            .collect();

        PProgram {
            system: self.system,
            toolchain: self.toolchain,
            opcodes,
        }
    }
}