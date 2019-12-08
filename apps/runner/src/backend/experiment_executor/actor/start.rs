use closure::*;
use log::*;

use lib_sandbox::SandboxListener;

use crate::backend::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub async fn start(mut self) {
        debug!("Actor started");
        debug!("-> experiment id: {}", self.assignment.experiment_id);
        debug!("-> experiment scenarios: {}", self.assignment.experiment_scenarios.len());

        let reporter = self.reporter.clone();

        self.sandbox.set_listener(SandboxListener {
            on_command_started: Some(box closure!(clone reporter, |cmd| {
                reporter.report_message(format!("Executing: {}", cmd));
            })),

            on_command_stdout: Some(box closure!(clone reporter, |line| {
                reporter.report_process_stdout(line);
            })),

            on_command_stderr: Some(box closure!(clone reporter, |line| {
                reporter.report_process_stderr(line);
            })),
        });

        self.reporter
            .report_experiment_started();

        let scenarios = self.assignment
            .experiment_scenarios
            .drain(..)
            .collect(): Vec<_>;

        for scenario in scenarios {
            self.process_messages().await;

            self.reporter
                .report_scenario_started();

            let scenario_succeeded = match self.execute_scenario(scenario).await {
                Ok(()) => {
                    true
                }

                Err(err) => {
                    self.reporter
                        .report_message(format!("Scenario failed: {}", err));

                    false
                }
            };

            self.reporter
                .report_scenario_completed(scenario_succeeded);
        }

        self.reporter
            .report_experiment_completed();

        debug!("Actor orphaned, halting");
    }
}