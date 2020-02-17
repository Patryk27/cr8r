use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::conv;
use crate::domain::{DomainError, DomainResult};
use crate::proto::models::PRunner;

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
    type Error = DomainError;

    fn try_from(PRunner { id, name, joined_at, last_heartbeat_at, status }: PRunner) -> DomainResult<Self> {
        Ok(Self {
            id: conv!(id as _),
            name: conv!(name as _),
            joined_at: conv!(joined_at as DateTime),
            last_heartbeat_at: conv!(last_heartbeat_at as DateTime),
            status: conv!(status? as _?),
        })
    }
}

impl Into<PRunner> for DRunner {
    fn into(self) -> PRunner {
        let Self { id, name, joined_at, last_heartbeat_at, status } = self;

        PRunner {
            id: conv!(id as _),
            name: conv!(name as _),
            joined_at: joined_at.to_rfc3339(),
            last_heartbeat_at: last_heartbeat_at.to_rfc3339(),
            status: Some(conv!(status as _)),
        }
    }
}