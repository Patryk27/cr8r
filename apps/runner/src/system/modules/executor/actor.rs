use anyhow::*;
use log::*;

use lib_core_channel::URx;
use lib_interop::models::{DEventType, DExperimentId};
use lib_sandbox::Sandbox;

use crate::rpc::Session;
use crate::system::{AttachmentStore, Logger};

use super::{ExecutorContext, ExecutorMsg, ExecutorStatus};

mod exec_experiment;
mod exec_job;
mod exec_opcode;
mod handle_messages;
mod init_context;

pub struct ExecutorActor {
    pub attachment_store: AttachmentStore,
    pub session: Session,
    pub sandbox: Sandbox,
    pub logger: Logger,
    pub experiment_id: DExperimentId,
    pub status: ExecutorStatus,
    pub mailbox: URx<ExecutorMsg>,
}

impl ExecutorActor {
    pub async fn start(mut self) {
        trace!("Actor started");
        trace!("-> experiment_id = {}", self.experiment_id);

        match self.try_start().await {
            Ok(_) => {
                self.handle_messages_until_orphaning().await
            }

            Err(err) => {
                // @todo we should notify controller about this incident
                error!("Experiment failed: {:#?}", err);

                self.status = ExecutorStatus::Completed;
                self.handle_messages_until_orphaning().await;
            }
        }

        trace!("Actor halted");
    }

    async fn try_start(&mut self) -> Result<()> {
        let context = self.init_context().await?;
        let workflow = self.exec_experiment(context).await;

        if workflow.actor_should_continue() {
            self.logger.add(DEventType::ExperimentCompleted);
            self.status = ExecutorStatus::Completed;
        } else {
            self.status = ExecutorStatus::Stopped;
        }

        Ok(())
    }
}