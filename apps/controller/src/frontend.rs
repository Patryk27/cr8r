use tonic::transport::Server;

use lib_protocol::controller::server::ControllerServer;

use crate::backend::System;
use crate::Result;

use self::service::ControllerService;

mod service;

pub async fn start(bind: String, system: System) -> Result<()> {
    let controller = ControllerService::new();

    Server::builder()
        .add_service(ControllerServer::new(controller))
        .serve(bind.parse().unwrap())
        .await
        .unwrap();

    Ok(())
}