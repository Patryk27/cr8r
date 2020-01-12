use log::*;

use lib_interop::domain::{DAssignment, DEventType};
use lib_sandbox::Sandbox;

use crate::experiment::{ExperimentExecutorStatus, ExperimentLogger};

use super::ExperimentExecutorRx;

mod execute_assignment;
mod execute_job;
mod execute_opcode;
mod handle_messages;

pub struct ExperimentExecutorActor {
    pub rx: ExperimentExecutorRx,
    pub sandbox: Sandbox,
    pub logger: ExperimentLogger,
    pub status: ExperimentExecutorStatus,
}

impl ExperimentExecutorActor {
    pub async fn start(mut self, assignment: DAssignment) {
        debug!("Actor started");

        self.logger.add(DEventType::ExperimentStarted);

        let workflow = self
            .execute_assignment(assignment)
            .await;

        if workflow.should_continue() {
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