use futures_channel::mpsc;

use lib_protocol::core::Report;

use crate::msg;

pub use self::{
    actor::*,
    command::*,
};

mod actor;
mod command;

#[derive(Clone, Debug)]
pub struct ExperimentWatcher {
    tx: ExperimentWatcherCommandTx,
}

impl ExperimentWatcher {
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExperimentWatcherActor::new(
            //
        ).start(rx));

        Self { tx }
    }

    pub fn add(&mut self, report: Report) {
        msg!(self.tx, ExperimentWatcherCommand::Add { report })
    }

    pub async fn get(&mut self) -> Option<Report> {
        msg!(self.tx, tx, ExperimentWatcherCommand::Get { tx })
    }

    pub fn kill(&mut self) {
        msg!(self.tx, ExperimentWatcherCommand::Kill)
    }
}