use futures_channel::mpsc;

use lib_protocol::core::{RunnerId, RunnerName};

use crate::backend::System;

pub use self::{
    actor::*,
    command::*,
};

mod actor;
mod command;

#[derive(Clone, Debug)]
pub struct Runner {
    tx: mpsc::UnboundedSender<RunnerCommand>,
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

    pub fn heartbeat(&self) {
        unimplemented!()
    }
}