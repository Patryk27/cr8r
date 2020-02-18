use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::conv;
use crate::models::{ModelError, ModelResult};
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
    type Error = ModelError;

    fn try_from(PExperiment { id, created_at, status }: PExperiment) -> ModelResult<Self> {
        Ok(Self {
            id: conv!(id as _),
            created_at: conv!(created_at as DateTime),
            status: conv!(status? as _?),
        })
    }
}

impl Into<PExperiment> for DExperiment {
    fn into(self) -> PExperiment {
        let Self { id, created_at, status } = self;

        PExperiment {
            id: conv!(id as _),
            created_at: created_at.to_rfc3339(),
            status: Some(conv!(status as _)),
        }
    }
}