use std::convert::TryFrom;

use crate::conv;
use crate::models::{ModelError, ModelResult};
use crate::proto::models::PJob;

pub use self::{
    id::*,
    name::*,
    opcode::DJobOpcode,
    status::*,
};

mod id;
mod name;
pub mod opcode;
mod status;

#[derive(Clone, Debug)]
pub struct DJob {
    pub id: DJobId,
    pub name: DJobName,
    pub opcodes: Vec<DJobOpcode>,
    pub status: DJobStatus,
}

impl TryFrom<PJob> for DJob {
    type Error = ModelError;

    fn try_from(PJob { id, name, opcodes, status }: PJob) -> ModelResult<Self> {
        Ok(Self {
            id: conv!(id as _),
            name: conv!(name as _),
            opcodes: conv!(opcodes as [_?]),
            status: conv!(status? as _?),
        })
    }
}

impl Into<PJob> for DJob {
    fn into(self) -> PJob {
        let Self { id, name, opcodes, status } = self;

        PJob {
            id: conv!(id as _),
            name: conv!(name as _),
            opcodes: conv!(opcodes as [_]),
            status: Some(conv!(status as _)),
        }
    }
}