use chrono::{DateTime, Utc};

use crate::contract::CProgram;
use crate::protocol::core::{PExperiment, PExperimentEvent};

pub use self::{
    def::*,
    event::*,
    id::*,
    report::*,
    status::*,
};

mod def;
mod event;
mod id;
mod report;
mod status;

#[derive(Clone, Debug)]
pub struct CExperiment {
    pub id: CExperimentId,
    pub program: CProgram,
    pub created_at: DateTime<Utc>,
    pub status: CExperimentStatus,
}

impl Into<PExperiment> for CExperiment {
    fn into(self) -> PExperiment {
        unimplemented!()
    }
}