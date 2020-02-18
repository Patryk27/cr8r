use anyhow::*;
use log::*;

use lib_core_channel::URx;
use lib_interop::models::{DEventType, DExperimentId};
use lib_sandbox::Sandbox;

use crate::rpc::Session;
use crate::system::{AttachmentStore, Logger};

use super::{ExecutorBehavior, ExecutorMsg, ExecutorStatus};

// mod download_attachments;
// mod execute_experiment;
// mod execute_job;
// mod execute_opcode;
// mod handle_messages;

pub struct ExecutorActor {
    pub session: Session,
    pub logger: Logger,
    pub experiment_id: DExperimentId,
    pub mailbox: URx<ExecutorMsg>,
}

impl ExecutorActor {
    pub async fn start(mut self, mut behavior: ExecutorBehavior) {
        trace!("Actor started");
        trace!("-> experiment_id = {}", self.experiment_id);

        loop {
            // @todo process messages

            match behavior.tick(&mut self).await {
                Some(next_behavior) => behavior = next_behavior,
                None => break,
            }
        }

        trace!("Actor halted");
    }

    // async fn try_start(&mut self) -> Result<()> {
    //     let context = {
    //         let attachments = self
    //             .download_attachments()
    //             .await
    //             .context("Could not download experiment's attachments")?;
    //
    //         let jobs = self.session
    //             .conn()
    //             .jobs()
    //             .find_many(self.experiment_id)
    //             .await
    //             .context("Could not fetch experiment's jobs")?;
    //
    //         ExecutorContext { attachments, jobs }
    //     };
    //
    //     let workflow = self
    //         .execute_experiment(context)
    //         .await;
    //
    //     if workflow.actor_should_continue() {
    //         self.logger.add(DEventType::ExperimentCompleted);
    //         self.status = ExecutorStatus::Completed;
    //     } else {
    //         // @todo notify logger?
    //         self.status = ExecutorStatus::Stopped;
    //     }
    //
    //     Ok(())
    // }
}