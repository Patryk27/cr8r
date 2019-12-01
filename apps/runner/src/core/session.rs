use log::*;

use lib_protocol::core::{Assignment, RunnerId, RunnerName};

use crate::{Client, Result};

#[derive(Clone)]
pub struct Session {
    client: Client,
    runner: RunnerId,
}

impl Session {
    pub async fn start(runner_name: RunnerName, controller_secret: String, mut client: Client) -> Result<Self> {
        info!("Opening session");

        // Ensure we're compatible with the controller
        debug!("Confirming protocol's compatibility");

        let version = client.hello()
            .await?
            .version;

        debug!("... controller's protocol version: {}", version);
        debug!("... ok, we should be compatible"); // @todo

        // Register us
        debug!("Registering");

        let runner = client.register(runner_name, controller_secret).await?.id;

        debug!("... We've been registered as: {}", runner);

        Ok(Self {
            client,
            runner,
        })
    }

    pub async fn ping(&mut self) -> Result<()> {
        self.client
            .ping(self.runner.clone())
            .await?;

        Ok(())
    }

    pub async fn request_assignment(&mut self) -> Result<Option<Assignment>> {
        let reply = self.client
            .request_assignment(self.runner.clone())
            .await?;

        Ok(reply.assignment)
    }
}