use tokio::sync::mpsc;

use lib_actor::ask;
use lib_interop::domain::DAssignment;
use lib_sandbox::Sandbox;

use crate::backend::Journalist;
use crate::core::ExperimentClient;

pub(self) use self::{
    actor::*,
    msg::*,
};
pub use self::status::*;

mod actor;
mod msg;
mod status;

pub struct Executor {
    tx: ExecutorTx,
}

impl Executor {
    pub fn new(sandbox: Sandbox, assignment: DAssignment, client: ExperimentClient) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let journalist = Journalist::new(client);

        tokio::spawn(ExecutorActor::new(
            rx,
            sandbox,
            assignment,
            journalist,
        ).main());

        Self { tx }
    }

    pub async fn get_status(&self) -> ExecutorStatus {
        ask!(self.tx, ExecutorMsg::GetStatus)
    }
}