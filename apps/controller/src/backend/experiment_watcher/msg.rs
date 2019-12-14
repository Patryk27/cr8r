use std::sync::Arc;

use futures_channel::{mpsc, oneshot};
use log::*;

use lib_protocol::core::PReport;
use lib_protocol::for_client::PWatchExperimentReply;

use crate::backend::experiment_watcher::ExperimentWatcherActor;

pub type ExperimentWatcherTx = mpsc::UnboundedSender<ExperimentWatcherMsg>;
pub type ExperimentWatcherRx = mpsc::UnboundedReceiver<ExperimentWatcherMsg>;

#[derive(Debug)]
pub enum ExperimentWatcherMsg {
    Kill,

    PullReply {
        tx: oneshot::Sender<PWatchExperimentReply>,
    },

    PushReport {
        report: Arc<PReport>,
    },
}

mod kill;
mod pull_reply;
mod push_report;

impl ExperimentWatcherMsg {
    pub fn process(self, actor: &mut ExperimentWatcherActor) {
        debug!("Processing message: {:?}", self);

        match self {
            ExperimentWatcherMsg::Kill => {
                kill::process(actor);
            }

            ExperimentWatcherMsg::PullReply { tx } => {
                pull_reply::process(actor, tx);
            }

            ExperimentWatcherMsg::PushReport { report } => {
                push_report::process(actor, report);
            }
        }
    }
}