use std::net::SocketAddr;

use anyhow::*;
use colored::Colorize;
use log::*;
use tonic::Interceptor;
use tonic::transport::Server;

use lib_core_ui::Logo;
use lib_interop::proto::services::{
    assignments_server::AssignmentsServer,
    attachments_server::AttachmentsServer,
    controller_server::ControllerServer,
    events_server::EventsServer,
    experiments_server::ExperimentsServer,
    jobs_server::JobsServer,
    reports_server::ReportsServer,
    runners_server::RunnersServer,
};

use crate::build;
use crate::system::System;

pub use self::config::*;

mod config;
mod interceptors;
mod services;

macro_rules! server {
    (
        $config:ident, $system:ident, $interceptor:ident, {
            $(
                for $svc:ident use $svc_handler:ident(
                    $($svc_param:ident),*
                ),
            )+
        }
    ) => {{
        let mut server = Server::builder();

        $(
            let server = server.add_service({
                let handler = $svc_handler {
                    $(
                        $svc_param: $system.$svc_param.clone(),
                    )*
                };

                let interceptor = $interceptor.clone();

                $svc::with_interceptor(handler, interceptor)
            });
        )+

        server
    }}
}

pub async fn start(config: RpcConfig, system: System) -> Result<()> {
    use self::{
        interceptors::*,
        services::*,
    };

    let address = config
        .address
        .parse()
        .context("Could not understand controller's address")?: SocketAddr;

    let interceptor: Interceptor = AuthorizingInterceptor::new(config.secret).into();

    let server = server!(config, system, interceptor, {
        for AssignmentsServer
        use AssignmentsService(experiment_store),

        for AttachmentsServer
        use AttachmentsService(attachment_store, experiment_store),

        for ControllerServer
        use ControllerService(),

        for EventsServer
        use EventsService(experiment_store),

        for ExperimentsServer
        use ExperimentsService(experiment_store),

        for JobsServer
        use JobsService(experiment_store),

        for ReportsServer
        use ReportsService(experiment_store),

        for RunnersServer
        use RunnersService(runner_store),
    });

    Logo {
        app: build::PKG_NAME,
        version: build::PKG_VERSION,
        commit: build::GIT_VERSION.unwrap(),
    }.log();

    info!(
        "🚀 Listening on: {}",
        address.to_string().green(),
    );

    server.serve(address).await?;

    Ok(())
}
