use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DExperiment, DJob, DomainError, DomainResult};
use crate::proto::core::PAssignment;

#[derive(Clone, Debug)]
pub struct DAssignment {
    pub experiment: DExperiment,
    pub jobs: Vec<DJob>,
}

impl TryFrom<PAssignment> for DAssignment {
    type Error = DomainError;

    fn try_from(PAssignment { experiment, jobs }: PAssignment) -> DomainResult<Self> {
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