use log::*;

use lib_interop::protocol::core::PAssignment;
use lib_sandbox::Sandbox;

use crate::backend::{ExperimentExecutorStatus, ExperimentJournalist};
use crate::backend::experiment_executor::ExperimentExecutorRx;

mod execute_experiment;
mod process_messages;

pub struct ExperimentExecutorActor {
    rx: ExperimentExecutorRx,
    pub(super) sandbox: Sandbox,
    pub(super) assignment: PAssignment,
    pub(super) journalist: ExperimentJournalist,
    pub(super) status: ExperimentExecutorStatus,
}

impl ExperimentExecutorActor {
    pub fn new(
        rx: ExperimentExecutorRx,
        sandbox: Sandbox,
        assignment: PAssignment,
        journalist: ExperimentJournalist,
    ) -> Self {
        Self {
            rx,
            sandbox,
            assignment,
            journalist,
            status: ExperimentExecutorStatus::Running,
        }
    }

    pub async fn main(mut self) {
        debug!("Actor started");

        self.journalist.add_experiment_started();

        // @todo
        self.process_messages_and_yield();

        if let Some(experiment) = self.assignment.experiment.take() {
            match self.execute_experiment(experiment).await {
                Ok(()) => {
                    self.journalist.add_experiment_succeeded();
                }

                Err(err) => {
                    self.journalist.add_experiment_failed(err);
                }
            };
        } else {
            self.journalist.add_experiment_failed("No experiment has been provided");
        }

        self.status = ExperimentExecutorStatus::Completed;

        self.process_messages_and_wait()
            .await;

        debug!("Actor finished working, halting");
    }
}