use chrono::Utc;
use tokio::{sync::mpsc, task};

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::domain::{DRunner, DRunnerId, DRunnerName};

use self::{
    actor::*,
    msg::*,
    status::*,
};

mod actor;
mod msg;
mod status;

#[derive(Clone)]
pub struct Runner {
    tx: UTx<RunnerMsg>,
}

impl Runner {
    pub fn new(id: DRunnerId, name: DRunnerName) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        task::spawn(RunnerActor {
            id,
            name,
            joined_at: Utc::now(),
            last_heartbeat_at: Utc::now(),
            status: Default::default(),
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