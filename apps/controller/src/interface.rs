use tonic::transport::Server;

use lib_protocol::client::server::ClientServer;
use lib_protocol::runner::server::RunnerServer;

use crate::Result;
use crate::system::System;

use self::services::*;

mod services;

pub async fn start(addr: String, system: System) -> Result<()> {
    Server::builder()
        .add_service(ClientServer::new(ClientService::new(system.clone())))
        .add_service(RunnerServer::new(RunnerService::new(system)))
        .serve(addr.parse()?)
        .await
        .unwrap();

    Ok(())
}