use lib_protocol_core::{Runner as RunnerModel, RunnerId, RunnerName, RunnerStatus};

use crate::backend::Runner;

pub struct RunnerEntry {
    pub runner: Runner,
    pub id: RunnerId,
    pub name: RunnerName,
    pub status: RunnerStatus,
}

impl RunnerEntry {
    pub fn as_model(&self) -> RunnerModel {
        RunnerModel {
            id: self.id,
            name: self.name.clone(),
            status: self.status.clone(),
        }
    }
}