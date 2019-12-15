use closure::*;
use log::*;

use lib_protocol::core::PAssignment;
use lib_sandbox::{Sandbox, SandboxListener};

use crate::backend::{ExperimentExecutorStatus, ExperimentJournalist};
use crate::backend::experiment_executor::ExperimentExecutorRx;

mod execute_scenario;
mod execute_step;
mod process_messages;

pub struct ExperimentExecutorActor {
    rx: ExperimentExecutorRx,
    pub(super) sandbox: Sandbox,
    pub(super) assignment: PAssignment,
    pub(super) reporter: ExperimentJournalist,
    pub(super) status: ExperimentExecutorStatus,
}

impl ExperimentExecutorActor {
    pub fn new(
        rx: ExperimentExecutorRx,
        sandbox: Sandbox,
        assignment: PAssignment,
        reporter: ExperimentJournalist,
    ) -> Self {
        Self {
            rx,
            sandbox,
            assignment,
            reporter,
            status: ExperimentExecutorStatus::Running,
        }
    }

    pub async fn main(mut self) {
        debug!("Actor started");
        debug!("-> experiment id: {}", self.assignment.experiment_id);
        debug!("-> experiment scenarios: {}", self.assignment.experiment_scenarios.len());

        let reporter = self.reporter.clone();

        self.sandbox.set_listener(SandboxListener {
            on_command_started: Some(box closure!(clone reporter, |cmd| {
                reporter.add_message(format!("Executing: {}", cmd));
            })),

            on_command_printed: Some(box closure!(clone reporter, |line| {
                reporter.add_process_output(line);
            })),
        });

        self.reporter.add_experiment_started();

        let scenarios = self.assignment
            .experiment_scenarios
            .drain(..)
            .collect(): Vec<_>;

        for scenario in scenarios {
            self.process_messages_yield();
            self.reporter.add_scenario_started();

            let success = match self.execute_scenario(scenario).await {
                Ok(()) => {
                    true
                }

                Err(err) => {
                    self.reporter.add_message(format!("Scenario failed: {}", err));
                    false
                }
            };

            self.reporter.add_scenario_completed(success);
        }

        self.reporter.add_experiment_completed();
        self.status = ExperimentExecutorStatus::Completed;

        self.process_messages_loop()
            .await;

        debug!("Actor orphaned, halting");
    }
}