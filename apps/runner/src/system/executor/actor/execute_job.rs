use anyhow::*;
use closure::*;
use log::*;

use lib_core_actor::*;
use lib_interop::domain::{DEventType, DJob, DJobOpcode};
use lib_sandbox::SandboxListener;

use super::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn execute_job(&mut self, job: DJob) -> Result<ActorWorkflow> {
        if self.handle_messages().actor_should_stop() {
            return Ok(ActorWorkflow::Stop);
        }

        self.init_sandbox()
            .await
            .context("Could not initialize sandbox")?;

        let result = self
            .execute_opcodes(job.opcodes)
            .await;

        if let Err(err) = self.destroy_sandbox().await {
            warn!("Could not destroy sandbox: {:?}", err);
            warn!("This may affect the next job");

            self.logger.add(DEventType::SystemMsg {
                msg: format!("Could not destroy sandbox: {:?}", err),
            });

            self.logger.add(DEventType::SystemMsg {
                msg: "This may affect the next job".to_string(),
            });
        }

        result
    }

    async fn init_sandbox(&mut self) -> Result<()> {
        self.logger.add(DEventType::SystemMsg {
            msg: "Initializing sandbox".to_string(),
        });

        let logger = self.logger.clone();

        let listener = SandboxListener {
            on_command_executed: Some(box closure!(clone logger, |cmd| {
                logger.add(DEventType::CustomMsg {
                    msg: format!("Executing: {}", cmd),
                });
            })),

            on_command_output: Some(box closure!(clone logger, |msg| {
                logger.add(DEventType::ProcessMsg {
                    msg,
                });
            })),
        };

        self.sandbox
            .init(listener)
            .await
    }

    async fn execute_opcodes(&mut self, opcodes: Vec<DJobOpcode>) -> Result<ActorWorkflow> {
        for (opcode_id, opcode) in opcodes.into_iter().enumerate() {
            debug!("Starting opcode [id={}]", opcode_id);

            if self.execute_opcode(opcode).await?.actor_should_stop() {
                return Ok(ActorWorkflow::Stop);
            }

            debug!("Completed opcode [id={}]", opcode_id);
        }

        Ok(ActorWorkflow::Continue)
    }

    async fn destroy_sandbox(&mut self) -> Result<()> {
        self.logger.add(DEventType::SystemMsg {
            msg: "Destroying sandbox".to_string(),
        });

        self.sandbox
            .destroy()
            .await
    }
}