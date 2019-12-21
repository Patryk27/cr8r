use closure::*;
use log::*;

use lib_interop::contract::{CAssignment, CEvent, CProgramOpcode};
use lib_sandbox::{Sandbox, SandboxListener};

use crate::backend::{ExecutorStatus, Journalist};
use crate::backend::executor::{ExecutorResult, ExperimentExecutorRx};

mod process_messages;

pub struct ExperimentExecutorActor {
    rx: ExperimentExecutorRx,
    pub(super) sandbox: Sandbox,
    pub(super) assignment: CAssignment,
    pub(super) journalist: Journalist,
    pub(super) status: ExecutorStatus,
}

impl ExperimentExecutorActor {
    pub fn new(
        rx: ExperimentExecutorRx,
        sandbox: Sandbox,
        assignment: CAssignment,
        journalist: Journalist,
    ) -> Self {
        Self {
            rx,
            sandbox,
            assignment,
            journalist,
            status: ExecutorStatus::Running,
        }
    }

    pub async fn main(mut self) {
        debug!("Actor started");

        self.journalist.add_event(CEvent::ExperimentStarted);

        // @todo
        self.process_messages_and_yield();

        let experiment_result: ExecutorResult<()> = try {
            self.init_sandbox()
                .await?;

            let opcodes = self.assignment
                .program
                .opcodes
                .drain(..)
                .collect(): Vec<_>;

            for (opcode_id, opcode) in opcodes.into_iter().enumerate() {
                let opcode_id = opcode_id as u32;

                match self.execute_opcode(opcode).await {
                    Ok(()) => {
                        self.journalist.add_event(CEvent::OpcodeSucceeded);
                    }

                    Err(err) => {
                        self.journalist.add_event(CEvent::OpcodeFailed {
                            id: opcode_id,
                            cause: err.to_string(),
                        });

                        Err(err)?;
                    }
                }
            }

            if let Err(err) = self.destroy_sandbox().await {
                error!("{}", err);
            }
        };

        match experiment_result {
            Ok(()) => {
                self.journalist.add_event(CEvent::ExperimentSucceeded);
            }

            Err(err) => {
                self.journalist.add_event(CEvent::ExperimentFailed {
                    cause: err.to_string(),
                });
            }
        };

        self.status = ExecutorStatus::Completed;

        self.process_messages_and_wait()
            .await;

        debug!("Actor finished working, halting");
    }

    async fn init_sandbox(&mut self) -> ExecutorResult<()> {
        self.journalist.add_event(CEvent::SystemMsg {
            msg: "Initializing sandbox".to_string(),
        });

        let journalist = self.journalist.clone();

        let listener = SandboxListener {
            on_command_executed: Some(box closure!(clone journalist, |cmd| {
                journalist.add_event(CEvent::UserMsg {
                    msg: format!("Executing: {}", cmd),
                });
            })),

            on_command_output: Some(box closure!(clone journalist, |line| {
                journalist.add_event(CEvent::ProcessOutput { line });
            })),
        };

        self.sandbox
            .init(Some(listener))
            .await
            .map_err(|err| format!("Couldn't initialize the sandbox: {}", err))
    }

    async fn execute_opcode(&mut self, opcode: CProgramOpcode) -> ExecutorResult<()> {
        self.process_messages_and_yield();

        match opcode {
            CProgramOpcode::LogSystemMsg { msg } => {
                self.journalist.add_event(CEvent::SystemMsg { msg });
            }

            CProgramOpcode::LogUserMsg { msg } => {
                self.journalist.add_event(CEvent::UserMsg { msg });
            }

            CProgramOpcode::Exec { cmd } => {
                self.sandbox
                    .exec(&cmd)
                    .await
                    .map_err(|err| err.to_string())?;
            }

            CProgramOpcode::PatchCrate { name, attachment_id } => {
                unimplemented!()
            }
        }

        Ok(())
    }

    async fn destroy_sandbox(&mut self) -> ExecutorResult<()> {
        self.journalist.add_event(CEvent::SystemMsg {
            msg: "Destroying sandbox".to_string(),
        });

        self.sandbox
            .destroy()
            .await
            .map_err(|err| format!("Couldn't destroy the sandbox: {}", err))
    }
}