use std::net::SocketAddr;

use colored::Colorize;
use hyper::{Body, Request, Response};
use log::*;
use tonic::body::BoxBody;
use tonic::transport::Server;
use tower::Service;

use lib_interop::protocol::{
    for_client::for_client_server::ForClientServer,
    for_runner::for_runner_server::ForRunnerServer,
};

use crate::backend::System;
use crate::core::Result;

use self::services::*;

mod services;

pub async fn start(addr: String, secret: Option<String>, system: System) -> Result<()> {
    let addr = addr.parse()?: SocketAddr;

    if secret.is_none() {
        warn!("You did not configure a secret key, so everybody will be able to connect to this controller");
    }

    info!("🚀 Listening on: {}", addr.to_string().green());

    Server::builder()
        .interceptor_fn(move |service, request| {
            let authorized = is_authorized(&secret, &request);
            let call = service.call(request);

            async move {
                if authorized {
                    call.await
                } else {
                    let res = Response::builder()
                        .header("grpc-status", "16")
                        .body(BoxBody::empty())?;

                    Ok(res)
                }
            }
        })
        .add_service(ForClientServer::new(ForClientService::new(system.clone())))
        .add_service(ForRunnerServer::new(ForRunnerService::new(system)))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}

fn is_authorized(secret: &Option<String>, request: &Request<Body>) -> bool {
    match secret {
        Some(secret) => {
            let recv_secret = request
                .headers()
                .get("authorization")
                .map(|h| h.to_str());

            if let Some(Ok(recv_secret)) = recv_secret {
                recv_secret == format!("Bearer {}", secret)
            } else {
                false
            }
        }

        None => {
            // No secret key = everyone can connect
            true
        }
    }
}