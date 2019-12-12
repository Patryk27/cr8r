use lib_protocol::core::p_scenario_step::*;
use lib_protocol::core::PScenarioStep;

use crate::backend::experiment_executor::{ExecutorResult, ExperimentExecutorActor};

impl ExperimentExecutorActor {
    pub(super) async fn execute_step(&mut self, step: PScenarioStep) -> ExecutorResult<()> {
        self.process_messages().await;

        if let Some(op) = step.op {
            match op {
                Op::Run(PRun { command }) => {
                    self.sandbox
                        .exec(&command)
                        .await
                        .map_err(|err| err.to_string())?;
                }

                Op::Print(PPrint { message }) => {
                    self.reporter.add_message(message);
                }
            }
        }

        Ok(())
    }
}