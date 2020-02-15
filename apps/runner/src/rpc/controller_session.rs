use anyhow::*;
use log::*;
use tonic::transport::Channel;

use lib_interop::connection::ControllerConnection;
use lib_interop::proto::models::{PRunnerId, PRunnerName};
use lib_interop::proto::services::{PHowdyRequest, PRegisterRunnerRequest};
use lib_interop::proto::services::assignments_client::AssignmentsClient;
use lib_interop::proto::services::events_client::EventsClient;

#[derive(Clone)]
pub struct Session {
    pub conn: ControllerConnection,
    pub runner_id: PRunnerId,
}

impl Session {
    pub async fn open(mut conn: ControllerConnection, runner_name: PRunnerName) -> Result<Self> {
        info!("Opening session");

        // Ensure we're compatible with the controller
        debug!("Confirming protocol's compatibility");

        let version = conn
            .controller()
            .howdy(PHowdyRequest {})
            .await?
            .into_inner()
            .version;

        debug!("... controller's protocol version: {}", version);
        debug!("... ok, we should be compatible"); // @todo

        // Register us
        debug!("Registering");

        let runner_id = conn
            .runners()
            .register_runner(PRegisterRunnerRequest { name: runner_name })
            .await?
            .into_inner()
            .id;

        debug!("... ok, we've been registered as id={}", runner_id);

        Ok(Self { conn, runner_id })
    }

    pub fn assignments(&self) -> AssignmentsClient<Channel> {
        self.conn.assignments()
    }

    pub fn events(&self) -> EventsClient<Channel> {
        self.conn.events()
    }
}
