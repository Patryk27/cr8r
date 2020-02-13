use tokio::{sync::mpsc, task};

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::domain::DAssignment;
use lib_sandbox::Sandbox;

use crate::rpc::ControllerSession;
use crate::system::Logger;

use self::{
    actor::*,
    msg::*,
};
pub use self::status::*;

mod actor;
mod msg;
mod status;

pub struct Executor {
    tx: UTx<ExecutorMsg>,
}

impl Executor {
    pub fn new(
        session: ControllerSession,
        assignment: DAssignment,
        sandbox: Sandbox,
        logger: Logger,
    ) -> Self {
        let (tx, mailbox) = mpsc::unbounded_channel();

        task::spawn(ExecutorActor {
            mailbox,
            sandbox,
            logger,
            status: ExecutorStatus::Running,
        }.start(assignment));

        Self { tx }
    }

    pub async fn get_status(&self) -> ExecutorStatus {
        ask!(self.tx, ExecutorMsg::GetStatus)
    }

    pub async fn stop(&self) {
        tell!(self.tx, ExecutorMsg::Stop);
    }
}