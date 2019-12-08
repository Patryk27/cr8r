use lib_protocol::core::Scenario;
use lib_sandbox::Sandbox;

use crate::backend::{ExecutorResult, ExperimentExecutorActor};

impl ExperimentExecutorActor {
    pub(super) async fn execute_scenario(&mut self, mut scenario: Scenario) -> ExecutorResult<()> {
        let result = try {
            self.launch_sandbox(&scenario)
                .await?;

            self.execute_steps(&mut scenario)
                .await?;
        };

        self.destroy_sandbox(&scenario)
            .await?;

        result
    }

    async fn launch_sandbox(&mut self, scenario: &Scenario) -> ExecutorResult<()> {
        self.reporter
            .report_message(format!("Preparing the sandbox (system: `{}`, toolchain: `{}`)", scenario.system, scenario.toolchain));

        self.sandbox
            .initialize(&scenario.system, &scenario.toolchain)
            .await
            .map_err(|err| format!("Failed to initialize the sandbox: {}", err))
    }

    async fn execute_steps(&mut self, scenario: &mut Scenario) -> ExecutorResult<()> {
        for step in scenario.steps.drain(..) {
            if let Err(err) = self.execute_step(step).await {
                return Err(format!("Step failed: {}", err));
            }
        }

        Ok(())
    }

    async fn destroy_sandbox(&mut self, scenario: &Scenario) -> ExecutorResult<()> {
        self.reporter
            .report_message("Destroying the sandbox");

        self.sandbox
            .destroy()
            .await
            .map_err(|err| format!("Failed to destroy the sandbox: {}", err))
    }
}