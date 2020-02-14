use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::proto::models::PExperiment;

pub use self::{
    id::*,
    status::*,
};

mod id;
mod status;

#[derive(Clone, Debug)]
pub struct DExperiment {
    pub id: DExperimentId,
    pub created_at: DateTime<Utc>,
    pub status: DExperimentStatus,
}

impl TryFrom<PExperiment> for DExperiment {
    type Error = DomainError;

    fn try_from(PExperiment { id, created_at, status }: PExperiment) -> DomainResult<Self> {
        Ok(Self {
            id: convert!(id as _),
            created_at: convert!(created_at as DateTime),
            status: convert!(status? as _?),
        })
    }
}

impl Into<PExperiment> for DExperiment {
    fn into(self) -> PExperiment {
        let Self { id, created_at, status } = self;

        PExperiment {
            id: convert!(id as _),
            created_at: created_at.to_rfc3339(),
            status: Some(convert!(status as _)),
        }
    }
}