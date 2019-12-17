use closure::*;
use log::*;

use lib_protocol::core::PScenario;
use lib_sandbox::SandboxListener;

use crate::backend::experiment_executor::{ExecutorResult, ExperimentExecutorActor};

impl ExperimentExecutorActor {
    pub(super) async fn execute_scenario(&mut self, mut scenario: PScenario) -> ExecutorResult<()> {
        let result = try {
            self.launch_sandbox()
                .await?;

            self.execute_steps(&mut scenario)
                .await?;
        };

        if let Err(err) = self.destroy_sandbox().await {
            error!("{}", err);
        }

        result
    }

    async fn launch_sandbox(&mut self) -> ExecutorResult<()> {
        self.journalist.add_message("Initializing sandbox");
        let reporter = self.journalist.clone();

        let listener = SandboxListener {
            on_command_executed: Some(box closure!(clone reporter, |cmd| {
                reporter.add_message(format!("Executing: {}", cmd));
            })),

            on_command_output: Some(box closure!(clone reporter, |line| {
                reporter.add_process_output(line);
            })),
        };

        self.sandbox
            .init(Some(listener))
            .await
            .map_err(|err| format!("Couldn't initialize the sandbox: {}", err))
    }

    async fn execute_steps(&mut self, scenario: &mut PScenario) -> ExecutorResult<()> {
        for step in scenario.steps.drain(..) {
            if let Err(err) = self.execute_step(step).await {
                return Err(format!("Step failed: {}", err));
            }
        }

        Ok(())
    }

    async fn destroy_sandbox(&mut self) -> ExecutorResult<()> {
        self.journalist.add_message("Destroying sandbox");

        self.sandbox
            .destroy()
            .await
            .map_err(|err| format!("Couldn't destroy the sandbox: {}", err))
    }
}