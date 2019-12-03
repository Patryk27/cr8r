use futures_channel::mpsc;

use lib_actor::{ask, tell};
use lib_protocol::core::Report;

pub use self::{
    actor::*,
    message::*,
};

mod actor;
mod message;

#[derive(Clone, Debug)]
pub struct ExperimentWatcher {
    tx: ExperimentWatcherTx,
}

impl ExperimentWatcher {
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExperimentWatcherActor::new(
            rx,
        ).start());

        Self { tx }
    }

    pub fn add(&mut self, report: Report) {
        tell!(self.tx, ExperimentWatcherMsg::Add { report })
    }

    pub async fn get(&mut self) -> Option<String> {
        ask!(self.tx, ExperimentWatcherMsg::Get)
    }

    pub fn kill(&mut self) {
        tell!(self.tx, ExperimentWatcherMsg::Kill)
    }
}