use anyhow::*;
use log::*;

use lib_sandbox::{SandboxConfig, SandboxProvider};

use crate::rpc::Session;

mod await_assignment;
mod conduct_assignment;

pub struct Dispatcher {
    pub sandbox_config: SandboxConfig,
    pub sandbox_provider: SandboxProvider,
    pub session: Session,
}

impl Dispatcher {
    pub async fn start(mut self) -> Result<()> {
        trace!("Actor started");

        loop {
            let assignment = self
                .await_assignment()
                .await;

            match self.conduct_assignment(assignment).await {
                Ok(_) => {
                    info!("Experiment conducted successfully");
                }

                Err(err) => {
                    // @todo we should notify controller about this incident
                    error!("Couldn't conduct experiment: {:?}", err);
                }
            }
        }
    }
}