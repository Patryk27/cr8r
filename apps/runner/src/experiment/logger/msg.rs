use log::*;
use tokio::sync::mpsc;

use lib_interop::domain::DEvent;

use super::ExperimentLoggerActor;

pub type ExperimentLoggerTx = mpsc::UnboundedSender<ExperimentLoggerMsg>;
pub type ExperimentLoggerRx = mpsc::UnboundedReceiver<ExperimentLoggerMsg>;

#[derive(Debug)]
pub enum ExperimentLoggerMsg {
    Add {
        event: DEvent,
    },
}

mod add;

impl ExperimentLoggerMsg {
    pub async fn handle(self, actor: &mut ExperimentLoggerActor) {
        use ExperimentLoggerMsg::*;

        debug!("Handling message: {:?}", self);

        match self {
            Add { event } => {
                add::add(actor, event)
                    .await;
            }
        }
    }
}