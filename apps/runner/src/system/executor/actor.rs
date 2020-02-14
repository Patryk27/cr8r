use log::*;

use lib_core_channel::URx;
use lib_interop::domain::{DEventType, DExperimentId};
use lib_sandbox::Sandbox;

use crate::rpc::ControllerSession;
use crate::system::{ExecutorStatus, Logger};

use super::ExecutorMsg;

mod execute_experiment;
mod execute_job;
mod execute_opcode;
mod handle_messages;

pub struct ExecutorActor {
    pub session: ControllerSession,
    pub sandbox: Sandbox,
    pub logger: Logger,
    pub mailbox: URx<ExecutorMsg>,
    pub experiment_id: DExperimentId,
    pub status: ExecutorStatus,
}

impl ExecutorActor {
    pub async fn start(mut self) {
        trace!("Actor started");
        trace!("-> experiment_id = {}", self.experiment_id);

        self.logger.add(DEventType::ExperimentStarted);

        let workflow = self
            .execute_experiment()
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