use std::str::FromStr;

use anyhow::*;
use tonic::client::Grpc;
use tonic::metadata::MetadataValue;
use tonic::Request;
use tonic::transport::{Channel, Uri};

use crate::proto::services::{
    assignments_client::AssignmentsClient,
    attachments_client::AttachmentsClient,
    controller_client::ControllerClient,
    events_client::EventsClient,
    experiments_client::ExperimentsClient,
    jobs_client::JobsClient,
    reports_client::ReportsClient,
};

#[derive(Clone)]
pub struct ControllerConnection {
    channel: Channel,
}

impl ControllerConnection {
    pub async fn new(address: String, secret: Option<String>) -> Result<Self> {
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

//        let dispatcher = Grpc::with_interceptor(channel, move |mut req: Request<()>| {
//            if let Some(auth) = &auth {
//                req.metadata_mut().insert("authorization", auth.clone());
//            }
//
//            Ok(req)
//        }); @todo

        Ok(Self { channel })
    }

    pub fn assignments(&self) -> AssignmentsClient<Channel> {
        AssignmentsClient::new(self.channel.clone())
    }

    pub fn attachments(&self) -> AttachmentsClient<Channel> {
        AttachmentsClient::new(self.channel.clone())
    }

    pub fn controller(&self) -> ControllerClient<Channel> {
        ControllerClient::new(self.channel.clone())
    }

    pub fn events(&self) -> EventsClient<Channel> {
        EventsClient::new(self.channel.clone())
    }

    pub fn experiments(&self) -> ExperimentsClient<Channel> {
        ExperimentsClient::new(self.channel.clone())
    }

    pub fn jobs(&self) -> JobsClient<Channel> {
        JobsClient::new(self.channel.clone())
    }

    pub fn reports(&self) -> ReportsClient<Channel> {
        ReportsClient::new(self.channel.clone())
    }
}