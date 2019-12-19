use log::*;

use lib_protocol::core::{PAssignment, PRunnerId, PRunnerName};

use crate::{Client, Result};
use crate::core::ExperimentClient;

#[derive(Clone)]
pub struct SessionClient {
    client: Client,
    runner: PRunnerId,
}

impl SessionClient {
    pub async fn start(runner_name: PRunnerName, controller_secret: String, mut client: Client) -> Result<Self> {
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

        debug!("... ok, we've been registered as: {}", runner);

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

    pub async fn request_assignment(&mut self) -> Result<Option<(PAssignment, ExperimentClient)>> {
        let reply = self.client
            .request_assignment(self.runner.clone())
            .await?;

        if let Some(assignment) = reply.assignment {
            let client = ExperimentClient::new(
                self.client.clone(),
                self.runner.clone(),
                assignment.experiment.as_ref().unwrap().id.clone(),
            );

            Ok(Some((assignment, client)))
        } else {
            Ok(None)
        }
    }
}