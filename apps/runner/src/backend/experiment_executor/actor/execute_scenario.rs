use lib_protocol::core::PScenario;

use crate::backend::experiment_executor::{ExecutorResult, ExperimentExecutorActor};

impl ExperimentExecutorActor {
    pub(super) async fn execute_scenario(&mut self, mut scenario: PScenario) -> ExecutorResult<()> {
        let result = try {
            self.launch_sandbox(&scenario)
                .await?;

            self.execute_steps(&mut scenario)
                .await?;
        };

        self.destroy_sandbox()
            .await?;

        result
    }

    async fn launch_sandbox(&mut self, scenario: &PScenario) -> ExecutorResult<()> {
        self.reporter.add_message(format!("Preparing sandbox (system `{}`, toolchain `{}`)", scenario.system, scenario.toolchain));

        self.sandbox
            .initialize(&scenario.system, &scenario.toolchain)
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
        self.reporter.add_message("Destroying sandbox");

        self.sandbox
            .destroy()
            .await
            .map_err(|err| format!("Couldn't destroy the sandbox: {}", err))
    }
}