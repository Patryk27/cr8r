use lib_protocol::core::{scenario_step, ScenarioStep};

use crate::backend::{ExecutorActor, ExecutorResult};

impl ExecutorActor {
    pub(super)async fn execute_step(&mut self, step: ScenarioStep) -> ExecutorResult<()> {
        self.process_messages().await;

        if let Some(op) = step.op {
            match op {
                scenario_step::Op::Exec(scenario_step::Exec { command }) => {
                    let success = await_lxd!(self, {
                        self.lxd.exec(&self.container, &[
                            "bash", "-c", &command,
                        ])
                    });

                    if !success {
                        return Err("Command returned a non-zero exit code".into());
                    }
                }

                scenario_step::Op::Print(scenario_step::Print { message }) => {
                    self.client
                        .report_message(message)
                        .await
                        .unwrap();
                }
            }
        }

        Ok(())
    }
}