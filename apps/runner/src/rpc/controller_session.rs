use anyhow::*;
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
        let version = conn
            .controller()
            .howdy(PHowdyRequest {})
            .await?
            .into_inner()
            .version;

        // @todo ensure we're compatible with controller

        let runner_id = conn
            .runners()
            .register_runner(PRegisterRunnerRequest { name: runner_name })
            .await?
            .into_inner()
            .id;

        Ok(Self { conn, runner_id })
    }

    pub fn assignments(&self) -> AssignmentsClient<Channel> {
        self.conn.assignments()
    }

    pub fn events(&self) -> EventsClient<Channel> {
        self.conn.events()
    }
}
