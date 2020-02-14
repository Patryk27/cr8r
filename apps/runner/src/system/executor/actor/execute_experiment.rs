use log::*;

use lib_core_actor::*;
use lib_interop::domain::{DEventType, DJob};

use super::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn execute_experiment(&mut self) -> ActorWorkflow {
        if self.handle_messages().actor_should_stop() {
            return ActorWorkflow::Stop;
        }

        // @todo download attachments

        let jobs: Vec<DJob> = unimplemented!();

        for (id, job) in jobs.into_iter().enumerate() {
            debug!("Starting job [id={}, name={}]", id, job.name);

            self.logger.add(DEventType::JobStarted { id });

            let result = self
                .execute_job(job)
                .await;

            match result {
                Ok(workflow) if workflow.actor_should_stop() => {
                    return ActorWorkflow::Stop;
                }

                result => {
                    debug!("Completed job [id={}]", id);

                    let result = result
                        .map(|_| ())
                        .map_err(|err| format!("{:#?}", err)); // @todo this could be nicer

                    self.logger.add(DEventType::JobCompleted { id, result });
                }
            }
        }

        ActorWorkflow::Continue
    }
}