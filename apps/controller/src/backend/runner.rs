use chrono::Utc;
use tokio::sync::mpsc;

use lib_core_actor::*;
use lib_interop::domain::{DRunner, DRunnerId, DRunnerName};

use crate::backend::System;

use self::{
    actor::*,
    msg::*,
    status::*,
};

mod actor;
mod msg;
mod status;

#[derive(Clone, Debug)]
pub struct Runner {
    tx: RunnerTx,
}

impl Runner {
    pub fn new(system: System, id: DRunnerId, name: DRunnerName) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(RunnerActor {
            system,
            id,
            name,
            joined_at: Utc::now(),
            last_heartbeat_at: Utc::now(),
            status: RunnerStatus::default(),
        }.start(rx));

        Self { tx }
    }

    pub async fn get_model(&self) -> DRunner {
        ask!(self.tx, RunnerMsg::GetModel)
    }

    pub fn kill(&self) {
        tell!(self.tx, RunnerMsg::Kill)
    }
}