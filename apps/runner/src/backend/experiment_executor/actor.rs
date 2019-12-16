use log::*;

use lib_protocol::core::PAssignment;
use lib_sandbox::Sandbox;

use crate::backend::{ExperimentExecutorStatus, ExperimentJournalist};
use crate::backend::experiment_executor::ExperimentExecutorRx;

mod execute_scenario;
mod execute_step;
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
        debug!("-> experiment id: {}", self.assignment.experiment_id);
        debug!("-> experiment scenarios: {}", self.assignment.experiment_scenarios.len());

        self.journalist.add_experiment_started();

        let scenarios = self.assignment
            .experiment_scenarios
            .drain(..)
            .collect(): Vec<_>;

        for scenario in scenarios {
            self.process_messages_yield();
            self.journalist.add_scenario_started();

            let success = match self.execute_scenario(scenario).await {
                Ok(()) => {
                    true
                }

                Err(err) => {
                    self.journalist.add_message(format!("Scenario failed: {}", err));
                    false
                }
            };

            self.journalist.add_scenario_completed(success);
        }

        self.journalist.add_experiment_completed();
        self.status = ExperimentExecutorStatus::Completed;

        self.process_messages_loop()
            .await;

        debug!("Actor orphaned, halting");
    }
}