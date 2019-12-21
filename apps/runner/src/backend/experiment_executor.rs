use tokio::sync::mpsc;

use lib_actor::ask;
use lib_interop::protocol::core::PAssignment;
use lib_sandbox::Sandbox;

use crate::backend::ExperimentJournalist;
use crate::core::ExperimentClient;

pub(self) use self::{
    actor::*,
    error::*,
    msg::*,
};
pub use self::status::*;

mod actor;
mod error;
mod msg;
mod status;

pub struct ExperimentExecutor {
    tx: ExperimentExecutorTx,
}

impl ExperimentExecutor {
    pub fn new(sandbox: Sandbox, assignment: PAssignment, client: ExperimentClient) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let journalist = ExperimentJournalist::new(client);

        tokio::spawn(ExperimentExecutorActor::new(
            rx,
            sandbox,
            assignment,
            journalist,
        ).main());

        Self { tx }
    }

    pub async fn get_status(&self) -> ExperimentExecutorStatus {
        ask!(self.tx, ExperimentExecutorMsg::GetStatus)
    }
}