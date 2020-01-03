use std::convert::TryFrom;

use crate::{convert, Error, Result};
use crate::domain::{DExperiment, DJob};
use crate::proto::core::PAssignment;

#[derive(Clone, Debug)]
pub struct DAssignment {
    pub experiment: DExperiment,
    pub jobs: Vec<DJob>,
}

impl TryFrom<PAssignment> for DAssignment {
    type Error = Error;

    fn try_from(PAssignment { experiment, jobs }: PAssignment) -> Result<Self> {
        Ok(Self {
            experiment: convert!(experiment? as _?),
            jobs: convert!(jobs as [_?]),
        })
    }
}

impl Into<PAssignment> for DAssignment {
    fn into(self) -> PAssignment {
        let Self { experiment, jobs } = self;

        PAssignment {
            experiment: Some(convert!(experiment as _)),
            jobs: convert!(jobs as [_]),
        }
    }
}