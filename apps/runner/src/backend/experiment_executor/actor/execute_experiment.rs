use closure::*;
use log::*;

use lib_protocol::core::{PExperiment, PExperimentStep};
use lib_protocol::core::p_experiment_step::*;
use lib_sandbox::SandboxListener;

use crate::backend::experiment_executor::{ExecutorResult, ExperimentExecutorActor};

impl ExperimentExecutorActor {
    pub(super) async fn execute_experiment(&mut self, experiment: PExperiment) -> ExecutorResult<()> {
        let result = try {
            self.init_sandbox()
                .await?;

            for (step_id, step) in experiment.steps.into_iter().enumerate() {
                let step_id = step_id as u32;

                match self.execute_step(step).await {
                    Ok(()) => {
                        self.journalist.add_step_succeeded(step_id);
                    }

                    Err(err) => {
                        self.journalist.add_step_failed(step_id, &err);
                        Err(err)?;
                    }
                }
            }
        };

        if let Err(err) = self.destroy_sandbox().await {
            error!("{}", err);
        }

        result
    }

    async fn init_sandbox(&mut self) -> ExecutorResult<()> {
        self.journalist.add_system_msg("Initializing sandbox");

        let journalist = self.journalist.clone();

        let listener = SandboxListener {
            on_command_executed: Some(box closure!(clone journalist, |cmd| {
                journalist.add_user_msg(format!("Executing: {}", cmd));
            })),

            on_command_output: Some(box closure!(clone journalist, |line| {
                journalist.add_process_output(line);
            })),
        };

        self.sandbox
            .init(Some(listener))
            .await
            .map_err(|err| format!("Couldn't initialize the sandbox: {}", err))
    }

    async fn execute_step(&mut self, step: PExperimentStep) -> ExecutorResult<()> {
        self.process_messages_and_yield();

        if let Some(op) = step.op {
            match op {
                Op::Exec(PExec { cmd }) => {
                    self.sandbox
                        .exec(&cmd)
                        .await
                        .map_err(|err| err.to_string())?;
                }

                Op::LogSystemMsg(PLogSystemMsg { msg }) => {
                    self.journalist.add_system_msg(msg);
                }

                Op::LogUserMsg(PLogUserMsg { msg }) => {
                    self.journalist.add_user_msg(msg);
                }
            }
        }

        Ok(())
    }

    async fn destroy_sandbox(&mut self) -> ExecutorResult<()> {
        self.journalist.add_system_msg("Destroying sandbox");

        self.sandbox
            .destroy()
            .await
            .map_err(|err| format!("Couldn't destroy the sandbox: {}", err))
    }
}