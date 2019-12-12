use futures_channel::{mpsc, oneshot};
use log::*;

use lib_protocol::core::PReport;

use crate::backend::experiment_watcher::ExperimentWatcherActor;

pub type ExperimentWatcherTx = mpsc::UnboundedSender<ExperimentWatcherMsg>;
pub type ExperimentWatcherRx = mpsc::UnboundedReceiver<ExperimentWatcherMsg>;

#[derive(Debug)]
pub enum ExperimentWatcherMsg {
    Add {
        report: PReport,
    },

    Get {
        tx: oneshot::Sender<Option<String>>,
    },

    Kill,
}

mod add;
mod get;
mod kill;

impl ExperimentWatcherMsg {
    pub fn process(self, actor: &mut ExperimentWatcherActor) {
        debug!("Processing message: {:?}", self);

        match self {
            ExperimentWatcherMsg::Add { report } => {
                add::process(actor, report);
            }

            ExperimentWatcherMsg::Get { tx } => {
                get::process(actor, tx);
            }

            ExperimentWatcherMsg::Kill => {
                kill::process(actor);
            }
        }
    }
}