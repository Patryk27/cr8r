use std::time::Duration;

use tokio::timer;

use lib_protocol::core::Scenario;

use crate::backend::{ExecutorActor, ExecutorResult};

impl ExecutorActor {
    pub(super) async fn execute_scenario(&mut self, mut scenario: Scenario) -> ExecutorResult<()> {
        self.process_messages().await;
        self.setup_system(&scenario).await?;

        let scenario_result: ExecutorResult<()> = try {
            // We have to wait a bit before systemd is initialized, otherwise we'll have whole lotta trouble with the
            // network connection). One second should be enough for all cases.
            timer::delay_for(Duration::from_millis(1000)).await;

            self.setup_toolchain(&scenario).await?;

            for step in scenario.steps.drain(..) {
                if let Err(err) = self.execute_step(step).await {
                    return Err(format!("Step failed: {}", err));
                }
            }
        };

        self.clean_up(&scenario).await;

        scenario_result
    }

    async fn setup_system(&mut self, scenario: &Scenario) -> ExecutorResult<()> {
        self.client
            .report_message(format!("Setting-up system (`{}`)", scenario.system))
            .await
            .unwrap();

        let success = await_lxd!(self, {
            self.lxd.launch(&self.container, &scenario.system)
        });

        if success {
            Ok(())
        } else {
            Err("Failed to set-up the system".into())
        }
    }

    async fn setup_toolchain(&mut self, scenario: &Scenario) -> ExecutorResult<()> {
        self.client
            .report_message(format!("Setting-up toolchain (`{}`)", scenario.toolchain))
            .await
            .unwrap();

        let success = await_lxd!(self, {
            self.lxd.exec(&self.container, &[
                "bash", "-c", "systemctl start network-online.target && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
            ])
        });

        if success {
            Ok(())
        } else {
            Err("Failed to set-up the toolchain".into())
        }
    }

    async fn clean_up(&mut self, scenario: &Scenario) {
        self.client
            .report_message("Cleaning-up after the experiment")
            .await
            .unwrap();

        // We don't care whether the `delete` succeeds or not, because we have to proceed as normal one way or another;
        // Or, rephrasing: even if we failed to delete the container, the show must go on.
        await_lxd!(self, {
            self.lxd.delete(&self.container)
        });
    }
}