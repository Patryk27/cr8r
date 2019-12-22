use std::convert::{TryFrom, TryInto};

use chrono::{DateTime, Utc};

use crate::{Error, parse, Result};
use crate::protocol::core::PExperiment;

pub use self::{
    def::*,
    id::*,
    status::*,
};

mod def;
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
            id: parse!(id as _),
            created_at: parse!(created_at as DateTime),
            status: parse!(status? as _?),
        })
    }
}

impl Into<PExperiment> for CExperiment {
    fn into(self) -> PExperiment {
        PExperiment {
            id: self.id.into(),
            created_at: self.created_at.to_rfc3339(),
            status: Some(self.status.into()),
        }
    }
}