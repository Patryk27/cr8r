use std::convert::{TryFrom, TryInto};

use crate::{Error, Result};
use crate::contract::{CExperiment, CProgram};
use crate::protocol::core::PAssignment;

#[derive(Clone, Debug)]
pub struct CAssignment {
    pub experiment: CExperiment,
    pub program: CProgram,
}

impl TryFrom<PAssignment> for CAssignment {
    type Error = Error;

    fn try_from(assignment: PAssignment) -> Result<Self> {
        Ok(Self {
            experiment: assignment.experiment
                .ok_or_else(|| Error::Missing { name: "experiment" })?
                .try_into()?,

            program: assignment.program
                .ok_or_else(|| Error::Missing { name: "program" })?
                .try_into()?,
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