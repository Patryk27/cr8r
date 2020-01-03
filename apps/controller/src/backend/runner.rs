use tokio::sync::mpsc;

use lib_actor::{ask, tell};
use lib_interop::domain::{DRunner, DRunnerId, DRunnerName};

use crate::backend::System;

pub(self) use self::{
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

        tokio::spawn(RunnerActor::new(
            rx,
            system,
            id,
            name,
        ).main());

        Self { tx }
    }

    pub async fn get_model(&self) -> DRunner {
        ask!(self.tx, RunnerMsg::GetModel)
    }

    pub fn kill(&self) {
        tell!(self.tx, RunnerMsg::Kill)
    }
}