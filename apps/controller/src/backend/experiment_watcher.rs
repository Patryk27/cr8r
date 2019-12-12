use futures_channel::mpsc;

use lib_actor::{ask, tell};
use lib_protocol::core::PReport;

pub(self) use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone, Debug)]
pub struct ExperimentWatcher {
    tx: ExperimentWatcherTx,
}

impl ExperimentWatcher {
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExperimentWatcherActor::new(
            rx,
        ).main());

        Self { tx }
    }

    pub fn add(&mut self, report: PReport) {
        tell!(self.tx, ExperimentWatcherMsg::Add { report })
    }

    pub async fn get(&mut self) -> Option<String> {
        ask!(self.tx, ExperimentWatcherMsg::Get)
    }

    pub fn kill(&mut self) {
        tell!(self.tx, ExperimentWatcherMsg::Kill)
    }
}