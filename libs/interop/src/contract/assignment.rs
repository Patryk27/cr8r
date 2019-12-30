use std::convert::TryFrom;

use crate::{convert, Error, Result};
use crate::contract::{CExperiment, CJob};
use crate::protocol::core::PAssignment;

#[derive(Clone, Debug)]
pub struct CAssignment {
    pub experiment: CExperiment,
    pub jobs: Vec<CJob>,
}

impl TryFrom<PAssignment> for CAssignment {
    type Error = Error;

    fn try_from(PAssignment { experiment, jobs }: PAssignment) -> Result<Self> {
        Ok(Self {
            experiment: convert!(experiment? as _?),
            jobs: convert!(jobs as [_?]),
        })
    }
}

impl Into<PAssignment> for CAssignment {
    fn into(self) -> PAssignment {
        let Self { experiment, jobs } = self;

        PAssignment {
            experiment: Some(convert!(experiment as _)),
            jobs: convert!(jobs as [_]),
        }
    }
}