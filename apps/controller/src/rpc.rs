use std::net::SocketAddr;

use anyhow::*;
use colored::Colorize;
use log::*;
use tonic::transport::Server;

use lib_interop::proto::controller::controller_server::ControllerServer;

use crate::system::System;

use self::{
    authorizer::*,
    service::*,
};
pub use self::config::*;

mod authorizer;
mod config;
mod service;

pub async fn start(config: RpcConfig, system: System) -> Result<()> {
    let address = config
        .address
        .parse()
        .context("Could not understand controller's address")?: SocketAddr;

    let service = ControllerServer::with_interceptor(
        ControllerService::new(system),
        Authorizer::new(config.secret),
    );

    info!("🚀 Listening on: {}", address.to_string().green());

    Server::builder()
        .add_service(service)
        .serve(address)
        .await?;

    Ok(())
}

