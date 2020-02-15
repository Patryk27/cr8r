use std::str::FromStr;

use anyhow::*;
use tonic::{Interceptor, Request};
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Uri};

use crate::proto::services::{
    assignments_client::AssignmentsClient,
    attachments_client::AttachmentsClient,
    controller_client::ControllerClient,
    events_client::EventsClient,
    experiments_client::ExperimentsClient,
    jobs_client::JobsClient,
    reports_client::ReportsClient,
    runners_client::RunnersClient,
};

#[derive(Clone)]
pub struct ControllerConnection {
    channel: Channel,
    interceptor: Interceptor,
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

        let interceptor = Interceptor::new(move |mut req: Request<()>| {
            if let Some(auth) = &auth {
                req.metadata_mut().insert("authorization", auth.clone());
            }

            Ok(req)
        });

        Ok(Self {
            channel,
            interceptor,
        })
    }

    pub fn assignments(&self) -> AssignmentsClient<Channel> {
        AssignmentsClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }

    pub fn attachments(&self) -> AttachmentsClient<Channel> {
        AttachmentsClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }

    pub fn controller(&self) -> ControllerClient<Channel> {
        ControllerClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }

    pub fn events(&self) -> EventsClient<Channel> {
        EventsClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }

    pub fn experiments(&self) -> ExperimentsClient<Channel> {
        ExperimentsClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }

    pub fn jobs(&self) -> JobsClient<Channel> {
        JobsClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }

    pub fn reports(&self) -> ReportsClient<Channel> {
        ReportsClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }

    pub fn runners(&self) -> RunnersClient<Channel> {
        RunnersClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }
}