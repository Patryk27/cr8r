use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::{convert, Error, Result};
use crate::proto::core::PRunner;

pub use self::{
    id::*,
    name::*,
    status::*,
};

mod id;
mod name;
mod status;

#[derive(Clone, Debug)]
pub struct DRunner {
    pub id: DRunnerId,
    pub name: DRunnerName,
    pub joined_at: DateTime<Utc>,
    pub last_heartbeat_at: DateTime<Utc>,
    pub status: DRunnerStatus,
}

impl TryFrom<PRunner> for DRunner {
    type Error = Error;

    fn try_from(PRunner { id, name, joined_at, last_heartbeat_at, status }: PRunner) -> Result<Self> {
        Ok(Self {
            id: convert!(id as _),
            name: convert!(name as _),
            joined_at: convert!(joined_at as DateTime),
            last_heartbeat_at: convert!(last_heartbeat_at as DateTime),
            status: convert!(status? as _?),
        })
    }
}

impl Into<PRunner> for DRunner {
    fn into(self) -> PRunner {
        let Self { id, name, joined_at, last_heartbeat_at, status } = self;

        PRunner {
            id: convert!(id as _),
            name: convert!(name as _),
            joined_at: joined_at.to_rfc3339(),
            last_heartbeat_at: last_heartbeat_at.to_rfc3339(),
            status: Some(convert!(status as _)),
        }
    }
}