use chrono::{DateTime, Utc};

use crate::protocol::core::PRunner;

pub use self::{
    id::*,
    name::*,
    status::*,
};

mod id;
mod name;
mod status;

#[derive(Clone, Debug)]
pub struct CRunner {
    pub id: CRunnerId,
    pub name: CRunnerName,
    pub joined_at: DateTime<Utc>,
    pub last_heartbeat_at: DateTime<Utc>,
    pub status: CRunnerStatus,
}

impl Into<PRunner> for CRunner {
    fn into(self) -> PRunner {
        PRunner {
            id: self.id.into(),
            name: self.name.into(),
            joined_at: self.joined_at.to_rfc3339(),
            last_heartbeat_at: self.last_heartbeat_at.to_rfc3339(),
            status: Some(self.status.into()),
        }
    }
}