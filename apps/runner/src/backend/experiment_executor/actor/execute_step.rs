use lib_protocol::core::{scenario_step, ScenarioStep};

use crate::backend::{ExecutorResult, ExperimentExecutorActor};

impl ExperimentExecutorActor {
    pub(super) async fn execute_step(&mut self, step: ScenarioStep) -> ExecutorResult<()> {
        self.process_messages().await;

        if let Some(op) = step.op {
            match op {
                scenario_step::Op::Exec(scenario_step::Exec { command }) => {
                    self.sandbox
                        .exec(&command)
                        .await
                        .map_err(|err| err.to_string())?;
                }

                scenario_step::Op::Print(scenario_step::Print { message }) => {
                    self.reporter
                        .report_message(message);
                }
            }
        }

        Ok(())
    }
}