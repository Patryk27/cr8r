use log::*;

use lib_interop::domain::DEvent;

use super::LoggerActor;

#[derive(Debug)]
pub enum LoggerMsg {
    Add {
        event: DEvent,
    },
}

mod add;

impl LoggerMsg {
    pub async fn handle(self, actor: &mut LoggerActor) {
        use LoggerMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            Add { event } => {
                add::add(actor, event)
                    .await;
            }
        }
    }
}