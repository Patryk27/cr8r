use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::{convert, Error, Result};
use crate::protocol::core::PExperiment;

pub use self::{
    definition::*,
    id::*,
    status::*,
};

mod definition;
mod id;
mod status;

#[derive(Clone, Debug)]
pub struct CExperiment {
    pub id: CExperimentId,
    pub created_at: DateTime<Utc>,
    pub status: CExperimentStatus,
}

impl TryFrom<PExperiment> for CExperiment {
    type Error = Error;

    fn try_from(PExperiment { id, created_at, status }: PExperiment) -> Result<Self> {
        Ok(Self {
            id: convert!(id as _),
            created_at: convert!(created_at as DateTime),
            status: convert!(status? as _?),
        })
    }
}

impl Into<PExperiment> for CExperiment {
    fn into(self) -> PExperiment {
        let Self { id, created_at, status } = self;

        PExperiment {
            id: convert!(id as _),
            created_at: created_at.to_rfc3339(),
            status: Some(convert!(status as _)),
        }
    }
}