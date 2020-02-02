use std::str::FromStr;

use anyhow::*;
use colored::Colorize;
use log::*;
use tonic::metadata::MetadataValue;
use tonic::Request;
use tonic::transport::{Channel, Uri};

use crate::proto::controller::*;
use crate::proto::controller::controller_client::ControllerClient as RawControllerClient;

mod assignment;
mod attachment;
mod event;
mod experiment;
mod report;
mod runner;

#[derive(Clone)]
pub struct ControllerClient {
    client: RawControllerClient<Channel>,
}

impl ControllerClient {
    pub async fn connect(address: String, secret: Option<String>) -> Result<Self> {
        info!("Connecting to controller at: {}", address.green());

        let uri = Uri::from_str(&address)
            .context("Could not understand controller's address")?;

        let auth = secret
            .map(|secret| format!("Bearer {}", secret))
            .map(|secret| MetadataValue::from_str(&secret))
            .transpose()
            .context("Could not understand controller's secret")?;

        let channel = Channel::builder(uri)
            .connect()
            .await?;

        let client = RawControllerClient::with_interceptor(channel, move |mut req: Request<()>| {
            if let Some(auth) = &auth {
                req.metadata_mut().insert("authorization", auth.clone());
            }

            Ok(req)
        });

        info!("Connection acquired");

        Ok(Self { client })
    }

    pub async fn howdy(&mut self) -> Result<PHowdyReply> {
        let response = self.client
            .howdy(PHowdyRequest {})
            .await?;

        Ok(response.into_inner())
    }
}