use std::net::SocketAddr;

use colored::Colorize;
use log::*;
use tonic::transport::Server;

use lib_protocol::for_client::server::ForClientServer;
use lib_protocol::for_runner::server::ForRunnerServer;

use crate::backend::System;
use crate::core::Result;

use self::services::*;

mod services;

pub async fn start(addr: String, system: System) -> Result<()> {
    let addr = addr.parse()?: SocketAddr;

    info!("🚀 Listening on: {}", addr.to_string().green());

    Server::builder()
        .add_service(ForClientServer::new(ForClientService::new(system.clone())))
        .add_service(ForRunnerServer::new(ForRunnerService::new(system)))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}