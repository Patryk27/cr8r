use std::convert::TryFrom;

use crate::conv;
use crate::models::{ModelError, ModelResult};
use crate::proto::models::PJob;

pub use self::{
    id::*,
    name::*,
    opcode::DJobOpcode,
};

mod id;
mod name;
pub mod opcode;

#[derive(Clone, Debug)]
pub struct DJob {
    pub id: DJobId,
    pub name: DJobName,
    pub opcodes: Vec<DJobOpcode>,
}

impl TryFrom<PJob> for DJob {
    type Error = ModelError;

    fn try_from(PJob { id, name, opcodes }: PJob) -> ModelResult<Self> {
        Ok(Self {
            id: conv!(id as _),
            name: conv!(name as _),
            opcodes: conv!(opcodes as [_?]),
        })
    }
}

impl Into<PJob> for DJob {
    fn into(self) -> PJob {
        let Self { id, name, opcodes } = self;

        PJob {
            id: conv!(id as _),
            name: conv!(name as _),
            opcodes: conv!(opcodes as [_]),
        }
    }
}