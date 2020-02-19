use chrono::Utc;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::models::{DRunner, DRunnerId, DRunnerName};

use crate::system::SystemEventBus;

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
    pub fn new(bus: SystemEventBus, id: DRunnerId, name: DRunnerName) -> Self {
        let (tx, rx) = unbounded_channel();

        spawn(RunnerActor {
            bus,
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

    pub async fn sync_heartbeat(&self) {
        ask!(self.tx, RunnerMsg::SyncHeartbeat);
    }
}