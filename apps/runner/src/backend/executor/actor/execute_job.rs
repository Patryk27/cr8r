use anyhow::{Context, Result};
use log::*;

use lib_interop::domain::{DEventType, DJob};

use crate::backend::executor::ExecutorActor;

mod destroy_sandbox;
mod execute_opcode;
mod init_sandbox;

impl ExecutorActor {
    pub(super) async fn execute_job(&mut self, job: DJob) -> Result<()> {
        self.init_sandbox()
            .await
            .context("Could not initialize the sandbox")?;

        let result = try {
            for opcode in job.opcodes {
                self.process_messages_and_yield()
                    .await;

                self.execute_opcode(opcode)
                    .await?;
            }
        };

        if let Err(err) = self.destroy_sandbox().await {
            warn!("Could not destroy the sandbox: {}", err);
            warn!("This may affect the next job");

            self.journalist.dispatch(DEventType::SystemMsg {
                msg: format!("Could not destroy sandbox: {}", err),
            });

            self.journalist.dispatch(DEventType::SystemMsg {
                msg: "This may affect the next job".to_string(),
            });
        }

        result
    }
}