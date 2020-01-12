use anyhow::Result;
use log::*;

use lib_sandbox::SandboxProvider;

use crate::core::SandboxConfig;
use crate::session::Session;

mod conduct_assignment;
mod get_assignment;

pub struct System {
    pub sandbox_config: SandboxConfig,
    pub sandbox_provider: SandboxProvider,
    pub session: Session,
}

impl System {
    pub async fn start(mut self) -> Result<()> {
        debug!("Actor started");

        loop {
            let assignment = self
                .get_assignment()
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