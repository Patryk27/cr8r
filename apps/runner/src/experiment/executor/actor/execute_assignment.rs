use lib_core_actor::*;
use lib_interop::domain::{DAssignment, DEventType};

use super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn execute_assignment(&mut self, assignment: DAssignment) -> ActorWorkflow {
        if self.handle_messages().should_stop() {
            return ActorWorkflow::Stop;
        }

        for (id, job) in assignment.jobs.into_iter().enumerate() {
            self.logger.add(DEventType::JobStarted { id });

            let result = self
                .execute_job(job)
                .await;

            match result {
                Ok(workflow) if workflow.should_stop() => {
                    return ActorWorkflow::Stop;
                }

                result => {
                    let result = result
                        .map(|_| ())
                        .map_err(|err| err.to_string());

                    self.logger.add(DEventType::JobCompleted { id, result });
                }
            }
        }

        ActorWorkflow::Continue
    }
}