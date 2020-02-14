use std::net::SocketAddr;

use anyhow::*;
use colored::Colorize;
use log::*;
use tonic::transport::Server;

use lib_interop::proto::services::assignments_server::AssignmentsServer;
use lib_interop::proto::services::controller_server::ControllerServer;

use crate::system::System;

pub use self::config::*;

mod config;
mod interceptors;
mod services;

pub async fn start(config: RpcConfig, system: System) -> Result<()> {
    use self::{
        interceptors::*,
        services::*,
    };

    let address = config
        .address
        .parse()
        .context("Could not understand controller's address")?: SocketAddr;

    let auth = AuthorizingInterceptor::new(config.secret);

    info!("🚀 Listening on: {}", address.to_string().green());

    let assignments = AssignmentsService::with_interceptor(AssignmentsService {
        experiments: system.experiments.clone(),
    }, auth.clone());

    // @todo
    Server::builder()
        .add_service(assignments)
        .serve(address)
        .await?;

    Ok(())
}

