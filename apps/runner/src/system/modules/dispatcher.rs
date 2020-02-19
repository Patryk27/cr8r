use anyhow::*;
use log::*;

use lib_sandbox::SandboxProvider;

use crate::rpc::Session;
use crate::system::AttachmentStore;

mod await_assignment;
mod conduct_assignment;

pub struct Dispatcher {
    pub sandbox_provider: SandboxProvider,
    pub attachment_store: AttachmentStore,
    pub session: Session,
}

impl Dispatcher {
    pub async fn start(mut self) -> Result<()> {
        trace!("Actor started");

        loop {
            let assignment = self.await_assignment().await;

            match self.conduct_assignment(assignment).await {
                Ok(_) => {
                    info!("Experiment completed");
                }

                Err(err) => {
                    // @todo we should notify controller about this incident
                    error!("Experiment failed: {:?}", err);
                }
            }
        }
    }
}