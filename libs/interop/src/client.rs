use std::str::FromStr;

use anyhow::*;
use colored::Colorize;
use log::*;
use tonic::{Request, Streaming};
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Uri};

use crate::proto::controller::*;
use crate::proto::controller::controller_client::ControllerClient as InnerControllerClient;
use crate::proto::core::*;

#[derive(Clone)]
pub struct ControllerClient {
    client: InnerControllerClient<Channel>,
}

impl ControllerClient {
    pub async fn connect(address: String, secret: Option<String>) -> Result<Self> {
        info!("Connecting to controller at: {}", address.green());

        let uri = Uri::from_str(&address)
            .context("Could not parse controller's address")?;

        let auth = secret
            .map(|secret| format!("Bearer {}", secret))
            .map(|secret| MetadataValue::from_str(&secret))
            .transpose()
            .context("Could not parse controller's secret")?;

        let channel = Channel::builder(uri)
            .connect()
            .await?;

        let client = InnerControllerClient::with_interceptor(channel, move |mut req: Request<()>| {
            if let Some(auth) = &auth {
                req.metadata_mut().insert("authorization", auth.clone());
            }

            Ok(req)
        });

        info!("Connection acquired");

        Ok(Self { client })
    }
}

/// Assignment-oriented impls
impl ControllerClient {
    pub async fn get_assignment(&mut self, runner_id: PRunnerId) -> Result<PGetAssignmentReply> {
        let response = self.client
            .get_assignment(PGetAssignmentRequest { runner_id })
            .await?;

        Ok(response.into_inner())
    }
}

/// Controller-oriented impls
impl ControllerClient {
    pub async fn howdy(&mut self) -> Result<PHowdyReply> {
        let response = self.client
            .howdy(PHowdyRequest {})
            .await?;

        Ok(response.into_inner())
    }
}

/// Event-oriented impls
impl ControllerClient {
    pub async fn add_event(
        &mut self,
        runner_id: PRunnerId,
        experiment_id: PExperimentId,
        event: PEvent,
    ) -> Result<PAddEventReply> {
        let response = self.client.add_event(PAddEventRequest {
            runner_id,
            experiment_id,
            event: Some(event),
        }).await?;

        Ok(response.into_inner())
    }
}

/// Experiment-oriented impls
impl ControllerClient {
    pub async fn create_experiment(&mut self, definition: PDefinition) -> Result<PCreateExperimentReply> {
        let response = self.client
            .create_experiment(PCreateExperimentRequest { definition: Some(definition) })
            .await?;

        Ok(response.into_inner())
    }

    pub async fn find_experiments(&mut self, request: PFindExperimentsRequest) -> Result<PFindExperimentsReply> {
        let response = self.client.find_experiments(request)
            .await?;

        Ok(response.into_inner())
    }

    pub async fn watch_experiment(&mut self, id: PExperimentId) -> Result<Streaming<PReport>> {
        let response = self.client
            .watch_experiment(PWatchExperimentRequest { id })
            .await?;

        Ok(response.into_inner())
    }
}

/// Report-oriented impls
impl ControllerClient {
    pub async fn find_reports(&mut self, request: PFindReportsRequest) -> Result<PFindReportsReply> {
        let response = self.client.find_reports(request)
            .await?;

        Ok(response.into_inner())
    }
}

/// Runner-oriented impls
impl ControllerClient {
    pub async fn find_runners(&mut self, request: PFindRunnersRequest) -> Result<PFindRunnersReply> {
        let response = self.client.find_runners(request)
            .await?;

        Ok(response.into_inner())
    }

    pub async fn register_runner(&mut self, name: PRunnerName) -> Result<PRegisterRunnerReply> {
        let response = self.client
            .register_runner(PRegisterRunnerRequest { name })
            .await?;

        Ok(response.into_inner())
    }
}