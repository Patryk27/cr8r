use log::*;
use tokio::sync::mpsc;

use lib_interop::domain::DEvent;

use crate::backend::journalist::JournalistActor;

pub type JournalistTx = mpsc::UnboundedSender<JournalistMsg>;
pub type JournalistRx = mpsc::UnboundedReceiver<JournalistMsg>;

#[derive(Debug)]
pub enum JournalistMsg {
    Dispatch {
        event: DEvent,
    },
}

mod dispatch;

impl JournalistMsg {
    pub async fn handle(self, actor: &mut JournalistActor) {
        use JournalistMsg::*;

        debug!("Handling message: {:?}", self);

        match self {
            Dispatch { event } => {
                dispatch::dispatch(actor, event)
                    .await;
            }
        }
    }
}