use anyhow::*;
use log::*;

use lib_core_actor::*;
use lib_interop::models::DEventType;

use super::{ExecutorActor, ExecutorContext};

impl ExecutorActor {
    pub(super) async fn exec_experiment(&mut self, mut context: ExecutorContext) -> ActorWorkflow {
        if self.handle_messages().actor_should_stop() {
            return ActorWorkflow::Stop;
        }

        while let Some(job) = context.jobs.pop_back() {
            let id = job.id;

            debug!("Starting job [id={}, name={}]", id, job.name);

            self.logger.add(DEventType::JobStarted { id });

            let result = self.exec_job(&context, job).await;

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

// @todo extract it somewhere
fn print_error(error: Error) -> String {
    let mut result = String::new();

    result.push_str(&error.to_string());

    for cause in error.chain().skip(1) {
        result.push_str(&format!(".. caused by: {}", cause.to_string()));
    }

    result
}