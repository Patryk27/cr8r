use anyhow::*;
use lib_core_actor::*;
use lib_interop::models::{DAttachmentId, DEventType, DJob};
use log::*;
use std::collections::HashMap;

use crate::system::Attachment;

use super::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn execute_experiment(&mut self, mut context: ExecutorContext) -> ActorWorkflow {
        if self.handle_messages().actor_should_stop() {
            return ActorWorkflow::Stop;
        }

        for (id, job) in jobs.into_iter().enumerate() {
            debug!("Starting job [id={}, name={}]", id, job.name);

            self.logger.add(DEventType::JobStarted { id });

            let result = self
                .execute_job(&attachments, job)
                .await;

            match result {
                Ok(workflow) if workflow.actor_should_stop() => {
                    return ActorWorkflow::Stop;
                }

                result => {
                    debug!("Completed job [id={}]", id);

                    let result = result
                        .map(|_| ())
                        .map_err(print_error);

                    self.logger.add(DEventType::JobCompleted { id, result });
                }
            }
        }

        ActorWorkflow::Continue
    }
}

fn print_error(error: Error) -> String {
    let mut result = String::new();

    result.push_str(&error.to_string());

    for cause in error.chain().skip(1) {
        result.push_str(&format!(".. caused by: {}", cause.to_string()));
    }

    result
}