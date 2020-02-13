use log::*;

use lib_core_channel::URx;
use lib_interop::domain::{DAssignment, DEventType};
use lib_sandbox::Sandbox;

use crate::system::{ExecutorStatus, Logger};

use super::ExecutorMsg;

mod execute_assignment;
mod execute_job;
mod execute_opcode;
mod handle_messages;

pub struct ExecutorActor {
    pub mailbox: URx<ExecutorMsg>,
    pub sandbox: Sandbox,
    pub logger: Logger,
    pub status: ExecutorStatus,
}

impl ExecutorActor {
    pub async fn start(mut self, assignment: DAssignment) {
        trace!("Actor started");

        self.logger.add(DEventType::ExperimentStarted);

        let workflow = self
            .execute_assignment(assignment)
            .await;

        if workflow.actor_should_continue() {
            self.logger.add(DEventType::ExperimentCompleted);
            self.status = ExecutorStatus::Completed;
        } else {
            // @todo notify logger?
            self.status = ExecutorStatus::Stopped;
        }

        self.handle_messages_until_orphaning()
            .await
    }
}