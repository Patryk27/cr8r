use chrono::Utc;
use tokio::sync::mpsc;

use lib_actor::tell;
use lib_interop::contract::{CEvent, CEventType};

use crate::core::ExperimentClient;

pub(self) use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct Journalist {
    tx: JournalistTx,
}

impl Journalist {
    pub fn new(client: ExperimentClient) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(JournalistActor::new(
            rx,
            client,
        ).main());

        Self { tx }
    }

    pub fn dispatch(&self, ty: CEventType) {
        let event = CEvent {
            at: Utc::now(),
            ty,
        };

        tell!(self.tx, JournalistMsg::Dispatch { event });
    }
}