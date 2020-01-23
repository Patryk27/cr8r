use std::net::SocketAddr;

use anyhow::{Context, Result};
use colored::Colorize;
use log::*;
use tonic::transport::Server;

use lib_interop::proto::controller::controller_server::ControllerServer;

use crate::backend::System;

use self::{
    authorizer::*,
    service::*,
};

mod authorizer;
mod service;

pub async fn start(address: String, secret: Option<String>, system: System) -> Result<()> {
    let address = address
        .parse()
        .context("Could not parse controller's address")?: SocketAddr;

    let service = ControllerServer::with_interceptor(
        ControllerService::new(system),
        Authorizer::new(secret),
    );

    info!("🚀 Listening on: {}", address.to_string().green());

    Server::builder()
        .add_service(service)
        .serve(address)
        .await?;

    Ok(())
}

