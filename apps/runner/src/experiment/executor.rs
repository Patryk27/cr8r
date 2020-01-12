use tokio::sync::mpsc;

use lib_actor::{ask, tell};
use lib_interop::domain::DAssignment;
use lib_sandbox::Sandbox;

use crate::experiment::ExperimentLogger;
use crate::session::Session;

use self::{
    actor::*,
    msg::*,
};
pub use self::status::*;

mod actor;
mod msg;
mod status;

pub struct ExperimentExecutor {
    tx: ExperimentExecutorTx,
}

impl ExperimentExecutor {
    pub fn new(
        session: Session,
        assignment: DAssignment,
        sandbox: Sandbox,
        logger: ExperimentLogger,
    ) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(ExperimentExecutorActor {
            rx,
            sandbox,
            logger,
            status: ExperimentExecutorStatus::Running,
        }.start(assignment));

        Self { tx }
    }

    pub async fn abort(&self) {
        tell!(self.tx, ExperimentExecutorMsg::Abort);
    }

    pub async fn get_status(&self) -> ExperimentExecutorStatus {
        ask!(self.tx, ExperimentExecutorMsg::GetStatus)
    }
}