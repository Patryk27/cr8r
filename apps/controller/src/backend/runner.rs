use futures_channel::mpsc;

use lib_actor::ask;
use lib_protocol::core::{PRunner, PRunnerId, PRunnerName};

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
    pub fn spawn(system: System, id: PRunnerId, name: PRunnerName) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(RunnerActor::new(
            rx,
            system,
            id,
            name,
        ).main());

        Self { tx }
    }

    pub async fn as_model(&self) -> PRunner {
        ask!(self.tx, RunnerMsg::AsModel)
    }
}