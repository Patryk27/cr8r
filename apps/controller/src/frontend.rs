use std::net::SocketAddr;

use colored::Colorize;
use log::*;
use tonic::transport::Server;
use tower::Service;

use lib_interop::protocol::{
    for_client::for_client_server::ForClientServer,
    for_runner::for_runner_server::ForRunnerServer,
};

use crate::backend::System;
use crate::core::Result;

use self::{
    middlewares::*,
    services::*,
};

mod middlewares;
mod services;

pub async fn start(addr: String, secret: Option<String>, system: System) -> Result<()> {
    let addr = addr.parse()?: SocketAddr;
    let auth = AuthorizeMiddleware::new(secret);

    info!("🚀 Listening on: {}", addr.to_string().green());

    Server::builder()
        .interceptor_fn(move |service, request| auth.handle(service, request))
        .add_service(ForClientServer::new(ForClientService::new(system.clone())))
        .add_service(ForRunnerServer::new(ForRunnerService::new(system)))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}