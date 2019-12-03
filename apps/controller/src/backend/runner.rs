use futures_channel::mpsc;

use lib_actor::ask;
use lib_protocol::core::{self, RunnerId, RunnerName};

use crate::backend::System;

pub use self::{
    actor::*,
    message::*,
};

mod actor;
mod message;

#[derive(Clone, Debug)]
pub struct Runner {
    tx: RunnerTx,
}

impl Runner {
    pub fn spawn(system: System, id: RunnerId, name: RunnerName) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(RunnerActor::new(
            rx,
            system,
            id,
            name,
        ).start());

        Self { tx }
    }

    pub async fn as_model(&self) -> core::Runner {
        ask!(self.tx, RunnerMsg::AsModel)
    }
}