use futures_channel::mpsc;

use lib_actor::ask;
use lib_protocol::core::Assignment;

use crate::core::ExperimentClient;

pub use self::{
    actor::*,
    message::*,
    status::*,
};

mod actor;
mod message;
mod status;

pub struct Executor {
    tx: ExecutorTx,
}

impl Executor {
    pub fn spawn(assignment: Assignment, client: ExperimentClient) -> Self {
        let (tx, rx) = mpsc::unbounded();

        tokio::spawn(ExecutorActor::new(
            rx,
            assignment,
            client,
        ).start());

        Self { tx }
    }

    pub async fn status(&self) {
        ask!(self.tx, ExecutorMsg::Status);
    }
}