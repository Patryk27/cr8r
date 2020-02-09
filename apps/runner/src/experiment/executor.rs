use tokio::{sync::mpsc, task};

use lib_core_actor::*;
use lib_core_channel::UTx;
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
    tx: UTx<ExperimentExecutorMsg>,
}

impl ExperimentExecutor {
    pub fn new(
        session: Session,
        assignment: DAssignment,
        sandbox: Sandbox,
        logger: ExperimentLogger,
    ) -> Self {
        let (tx, mailbox) = mpsc::unbounded_channel();

        task::spawn(ExperimentExecutorActor {
            mailbox,
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