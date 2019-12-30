use closure::*;
use log::*;

use lib_interop::contract::{CEventType, CJob, CJobOpcode};
use lib_sandbox::SandboxListener;

use crate::backend::executor::{ExecutorActor, ExecutorResult};

impl ExecutorActor {
    pub(super) async fn perform_job(&mut self, job: CJob) -> ExecutorResult<()> {
        self.init_sandbox()
            .await?;

        let result = try {
            for opcode in job.opcodes {
                self.process_messages_and_yield()
                    .await;

                self.perform_opcode(opcode)
                    .await?;
            }
        };

        if let Err(err) = self.destroy_sandbox().await {
            warn!("Could not destroy sandbox: {}", err);
            warn!("This may affect the next job");

            self.journalist.dispatch(CEventType::SystemMsg {
                msg: format!("Could not destroy sandbox: {}", err),
            });

            self.journalist.dispatch(CEventType::SystemMsg {
                msg: "This may affect the next job".to_string(),
            });
        }

        result
    }

    async fn init_sandbox(&mut self) -> ExecutorResult<()> {
        self.journalist.dispatch(CEventType::SystemMsg {
            msg: "Initializing sandbox".to_string(),
        });

        let journalist = self.journalist.clone();

        let listener = SandboxListener {
            on_command_executed: Some(box closure!(clone journalist, |cmd| {
                journalist.dispatch(CEventType::UserMsg {
                    msg: format!("Executing: {}", cmd),
                });
            })),

            on_command_output: Some(box closure!(clone journalist, |line| {
                journalist.dispatch(CEventType::ProcessOutput { line });
            })),
        };

        self.sandbox
            .init(Some(listener))
            .await
            .map_err(|err| format!("Couldn't initialize the sandbox: {}", err))
    }

    async fn perform_opcode(&mut self, opcode: CJobOpcode) -> ExecutorResult<()> {
        match opcode {
            CJobOpcode::LogSystemMsg { msg } => {
                self.journalist.dispatch(CEventType::SystemMsg { msg });
            }

            CJobOpcode::LogUserMsg { msg } => {
                self.journalist.dispatch(CEventType::UserMsg { msg });
            }

            CJobOpcode::Exec { cmd } => {
                self.sandbox
                    .exec(&cmd)
                    .await
                    .map_err(|err| err.to_string())?;
            }

            CJobOpcode::PatchCrate { .. } => {
                unimplemented!()
            }
        }

        Ok(())
    }

    async fn destroy_sandbox(&mut self) -> ExecutorResult<()> {
        self.journalist.dispatch(CEventType::SystemMsg {
            msg: "Destroying sandbox".to_string(),
        });

        self.sandbox
            .destroy()
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }
}