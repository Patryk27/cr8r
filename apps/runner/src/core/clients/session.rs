use log::*;

use lib_interop::client::ControllerClient;
use lib_interop::proto::core::{PAssignment, PRunnerId, PRunnerName};

use crate::core::ExperimentClient;
use crate::Result;

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
            .await
            .unwrap()
            .version;

        debug!("... controller's protocol version: {}", version);
        debug!("... ok, we should be compatible"); // @todo

        // Register us
        debug!("Registering");

        let id = client.register_runner(name)
            .await
            .unwrap()
            .id;

        debug!("... ok, we've been registered as: {}", id);

        Ok(Self { client, runner: id })
    }

    pub async fn get_assignment(&mut self) -> Result<Option<(PAssignment, ExperimentClient)>> {
        let reply = self.client
            .get_assignment(self.runner.clone())
            .await
            .unwrap();

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
