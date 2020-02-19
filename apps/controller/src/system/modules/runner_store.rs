use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::models::{DRunnerId, DRunnerName};

use crate::system::{Runner, SystemEventBus};

use self::{
    actor::*,
    error::*,
    msg::*,
};

mod actor;
mod error;
mod msg;

#[derive(Clone)]
pub struct RunnerStore {
    tx: UTx<RunnerStoreMsg>,
}

impl RunnerStore {
    pub fn new(bus: SystemEventBus) -> Self {
        let (tx, rx) = unbounded_channel();

        spawn(RunnerStoreActor {
            bus,
            index: Default::default(),
            runners: Default::default(),
            next_id: Default::default(),
        }.start(rx));

        Self { tx }
    }

    pub async fn find_all(&self) -> Vec<Runner> {
        ask!(self.tx, RunnerStoreMsg::FindAll)
    }

    pub async fn find_one(&self, id: DRunnerId) -> Result<Runner> {
        ask!(self.tx, RunnerStoreMsg::FindOne { id })
    }

    pub async fn register(&self, name: DRunnerName) -> Result<DRunnerId> {
        ask!(self.tx, RunnerStoreMsg::Register { name })
    }
}
