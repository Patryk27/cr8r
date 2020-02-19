use std::str::FromStr;

use anyhow::*;
use tonic::{Interceptor, Request};
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Uri};

use crate::clients::*;

#[derive(Clone)]
pub struct Connection {
    crate channel: Channel,
    crate interceptor: Interceptor,
}

impl Connection {
    pub async fn new(address: String, secret: Option<String>) -> Result<Self> {
        let uri = Uri::from_str(&address)
            .context("Could not understand controller's address")?;

        let auth = secret
            .map(|secret| format!("Bearer {}", secret))
            .map(|secret| MetadataValue::from_str(&secret))
            .transpose()
            .context("Could not understand controller's secret")?;

        let channel = Channel::builder(uri)
            .connect().await?;

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

    pub fn assignments(&self) -> AssignmentClient {
        AssignmentClient::new(self.clone())
    }

    pub fn attachments(&self) -> AttachmentClient {
        AttachmentClient::new(self.clone())
    }

    pub fn controller(&self) -> ControllerClient {
        ControllerClient::new(self.clone())
    }

    pub fn events(&self) -> EventClient {
        EventClient::new(self.clone())
    }

    pub fn experiments(&self) -> ExperimentClient {
        ExperimentClient::new(self.clone())
    }

    pub fn jobs(&self) -> JobClient {
        JobClient::new(self.clone())
    }

    pub fn reports(&self) -> ReportClient {
        ReportClient::new(self.clone())
    }

    pub fn runners(&self) -> RunnerClient {
        RunnerClient::new(self.clone())
    }
}