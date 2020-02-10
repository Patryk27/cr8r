use log::*;

use lib_core_channel::URx;
use lib_interop::domain::{DAssignment, DEventType};
use lib_sandbox::Sandbox;

use crate::experiment::{ExperimentExecutorStatus, ExperimentLogger};

use super::ExperimentExecutorMsg;

mod execute_assignment;
mod execute_job;
mod execute_opcode;
mod handle_messages;

pub struct ExperimentExecutorActor {
    pub mailbox: URx<ExperimentExecutorMsg>,
    pub sandbox: Sandbox,
    pub logger: ExperimentLogger,
    pub status: ExperimentExecutorStatus,
}

impl ExperimentExecutorActor {
    pub async fn start(mut self, assignment: DAssignment) {
        trace!("Actor started");

        self.logger.add(DEventType::ExperimentStarted);

        let workflow = self
            .execute_assignment(assignment)
            .await;

        if workflow.actor_should_continue() {
            self.logger.add(DEventType::ExperimentCompleted);
            self.status = ExperimentExecutorStatus::Completed;
        } else {
            // @todo notify logger?
            self.status = ExperimentExecutorStatus::Aborted;
        }

        self.handle_messages_until_orphaning()
            .await
    }
}