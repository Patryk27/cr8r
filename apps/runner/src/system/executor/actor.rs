use anyhow::*;
use log::*;

use lib_core_channel::URx;
use lib_interop::domain::{DEventType, DExperimentId};
use lib_sandbox::Sandbox;

use crate::rpc::ControllerSession;
use crate::system::{AttachmentStore, ExecutorStatus, Logger};

use super::ExecutorMsg;

mod execute_experiment;
mod execute_job;
mod execute_opcode;
mod fetch_attachments;
mod fetch_jobs;
mod handle_messages;

pub struct ExecutorActor {
    pub attachment_store: AttachmentStore,
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

        let result = try {
            let attachments = self
                .fetch_attachments()
                .await
                .context("Could not download experiment's attachments")?;

            let jobs = self
                .fetch_jobs()
                .await
                .context("Could not fetch experiment's jobs")?;

            let workflow = self
                .execute_experiment(attachments, jobs)
                .await;

            if workflow.actor_should_continue() {
                self.logger.add(DEventType::ExperimentCompleted);
                self.status = ExecutorStatus::Completed;
            } else {
                // @todo notify logger?
                self.status = ExecutorStatus::Stopped;
            }
        }: Result<_>;

        match result {
            Ok(()) => {
                self.handle_messages_until_orphaning()
                    .await
            }

            Err(_) => {
                // @todo notify controller? try again in a minute? give up?
                unimplemented!()
            }
        }

        trace!("Actor halted");
    }
}