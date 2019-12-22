use std::convert::{TryFrom, TryInto};

use crate::{Error, parse, Result};
use crate::contract::{CExperiment, CProgram};
use crate::protocol::core::PAssignment;

#[derive(Clone, Debug)]
pub struct CAssignment {
    pub experiment: CExperiment,
    pub program: CProgram,
}

impl TryFrom<PAssignment> for CAssignment {
    type Error = Error;

    fn try_from(PAssignment { experiment, program }: PAssignment) -> Result<Self> {
        Ok(Self {
            experiment: parse!(experiment? as _?),
            program: parse!(program? as _?),
        })
    }
}

impl Into<PAssignment> for CAssignment {
    fn into(self) -> PAssignment {
        PAssignment {
            experiment: Some(self.experiment.into()),
            program: Some(self.program.into()),
        }
    }
}