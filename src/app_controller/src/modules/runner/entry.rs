use actix::Addr;

use lib_protocol::{Runner, RunnerId, RunnerName, RunnerStatus};

use crate::modules::RunnerActor;

pub struct RunnerEntry {
    pub actor: Addr<RunnerActor>,
    pub id: RunnerId,
    pub name: RunnerName,
    pub status: RunnerStatus,
}

impl RunnerEntry {
    pub fn as_model(&self) -> Runner {
        Runner {
            id: self.id,
            name: self.name.clone(),
            status: self.status.clone(),
        }
    }
}