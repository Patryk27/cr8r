use std::net::SocketAddr;

use colored::Colorize;
use log::*;
use tonic::transport::Server;

use lib_protocol::client::server::ClientServer;
use lib_protocol::runner::server::RunnerServer;

use crate::backend::System;
use crate::core::Result;

use self::services::*;

mod services;

pub async fn start(addr: String, system: System) -> Result<()> {
    let addr = addr.parse()?: SocketAddr;

    info!("Listening on: {}", addr.to_string().green());

    Server::builder()
        .add_service(ClientServer::new(ClientService::new(system.clone())))
        .add_service(RunnerServer::new(RunnerService::new(system)))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}