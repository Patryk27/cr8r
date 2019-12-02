use futures_channel::mpsc;

use lib_protocol::core::{self, RunnerId, RunnerName};

use crate::backend::System;
use crate::msg;

pub use self::{
    actor::*,
    command::*,
};

mod actor;
mod command;

#[derive(Clone, Debug)]
pub struct Runner {
    tx: RunnerCommandTx,
}

impl Runner {
    pub fn spawn(system: System, id: RunnerId, name: RunnerName) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(RunnerActor::new(
            system,
            id,
            name,
        ).start(rx));

        Self { tx }
    }

    pub async fn as_model(&self) -> core::Runner {
        msg!(self.tx, tx, RunnerCommand::AsModel { tx })
    }
}