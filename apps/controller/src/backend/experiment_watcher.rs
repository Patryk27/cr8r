use std::sync::Arc;

use futures_channel::mpsc;

use lib_actor::{ask, tell};
use lib_protocol::core::PReport;
use lib_protocol::for_client::PWatchExperimentReply;

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

    pub fn push_report(&mut self, report: Arc<PReport>) {
        tell!(self.tx, ExperimentWatcherMsg::PushReport { report })
    }

    pub async fn pull_reply(&mut self) -> PWatchExperimentReply {
        ask!(self.tx, ExperimentWatcherMsg::PullReply)
    }

    pub fn kill(&mut self) {
        tell!(self.tx, ExperimentWatcherMsg::Kill)
    }
}