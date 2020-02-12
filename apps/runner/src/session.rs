use std::future::Future;

use anyhow::*;
use log::*;

use lib_interop::client::ControllerClient;
use lib_interop::proto::core::{PRunnerId, PRunnerName};

#[derive(Clone)]
pub struct Session {
    pub client: ControllerClient,
    pub runner_id: PRunnerId,
}

impl Session {
    pub async fn open(mut client: ControllerClient, name: PRunnerName) -> Result<Self> {
        info!("Opening session");

        // Ensure we're compatible with the controller
        debug!("Confirming protocol's compatibility");

        let version = client.howdy()
            .await?
            .version;

        debug!("... controller's protocol version: {}", version);
        debug!("... ok, we should be compatible"); // @todo

        // Register us
        debug!("Registering");

        let runner_id = client.register_runner(name)
            .await?
            .id;

        debug!("... ok, we've been registered as: {}", runner_id);

        Ok(Self { client, runner_id })
    }

    pub fn invoke<'a, T, F: Future<Output=T>>(
        &'a mut self,
        f: impl FnOnce(&'a mut ControllerClient, PRunnerId) -> F,
    ) -> F {
        f(&mut self.client, self.runner_id)
    }
}
