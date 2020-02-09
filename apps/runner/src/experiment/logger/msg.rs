use log::*;

use lib_interop::domain::DEvent;

use super::ExperimentLoggerActor;

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