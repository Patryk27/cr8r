use std::convert::{TryFrom, TryInto};

use chrono::{DateTime, Utc};
use snafu::ResultExt;

use crate::{error, Error, Result};
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

    fn try_from(experiment: PExperiment) -> Result<Self> {
        Ok(Self {
            id: experiment.id.into(),

            created_at: DateTime::parse_from_rfc3339(&experiment.created_at)
                .context(error::InvalidDateTime { name: "created_at" })?
                .with_timezone(&Utc),

            status: experiment.status
                .ok_or_else(|| Error::Missing { name: "status" })?
                .try_into()?,
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