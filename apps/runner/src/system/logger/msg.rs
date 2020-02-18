use log::*;

use lib_interop::models::DEvent;

use super::LoggerActor;

mod add;

#[derive(Debug)]
pub enum LoggerMsg {
    Add {
        event: DEvent,
    },
}

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