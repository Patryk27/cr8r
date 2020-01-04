use anyhow::Result;
use log::*;

use lib_interop::client::ControllerClient;
use lib_interop::proto::core::{PAssignment, PRunnerId, PRunnerName};

use crate::core::ExperimentClient;

#[derive(Clone)]
pub struct SessionClient {
    client: ControllerClient,
    runner: PRunnerId,
}

impl SessionClient {
    pub async fn start(name: PRunnerName, mut client: ControllerClient) -> Result<Self> {
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

        let id = client.register_runner(name)
            .await?
            .id;

        debug!("... ok, we've been registered as: {}", id);

        Ok(Self { client, runner: id })
    }

    pub async fn get_assignment(&mut self) -> Result<Option<(PAssignment, ExperimentClient)>> {
        let reply = self.client
            .get_assignment(self.runner.clone())
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
