use std::future::Future;

use anyhow::*;
use log::*;
use tonic::client::Grpc;
use tonic::transport::Channel;

use lib_interop::connection::ControllerConnection;
use lib_interop::proto::models::{PRunnerId, PRunnerName};
use lib_interop::proto::services::assignments_client::AssignmentsClient;
use lib_interop::proto::services::events_client::EventsClient;

#[derive(Clone)]
pub struct Session {
    pub conn: ControllerConnection,
    pub runner_id: PRunnerId,
}

impl Session {
    pub async fn open(mut conn: ControllerConnection, runner_name: PRunnerName) -> Result<Self> {
        unimplemented!()

//        info!("Opening session");
//
//        // Ensure we're compatible with the controller
//        debug!("Confirming protocol's compatibility");
//
//        let version = client
//            .howdy()
//            .await?
//            .version;
//
//        debug!("... controller's protocol version: {}", version);
//        debug!("... ok, we should be compatible"); // @todo
//
//        // Register us
//        debug!("Registering");
//
//        let runner_id = client
//            .register_runner(runner_name)
//            .await?
//            .id;
//
//        debug!("... ok, we've been registered as: {}", runner_id);
//
//        unimplemented!()
    }

    pub fn assignments(&self) -> AssignmentsClient<Channel> {
        self.conn.assignments()
    }

    pub fn events(&self) -> EventsClient<Channel> {
        self.conn.events()
    }
}
