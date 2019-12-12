use colored::Colorize;
use log::*;
use tonic::transport::Channel;

use lib_protocol::core::{PExperimentId, PReport, PRunnerId};
use lib_protocol::for_runner::*;
use lib_protocol::for_runner::client::ForRunnerClient;

use crate::Result;

pub use self::scopes::*;

mod scopes;

#[derive(Clone)]
pub struct Client {
    client: ForRunnerClient<Channel>,
}

impl Client {
    pub async fn connect(address: String) -> Result<Self> {
        info!("Connecting to controller at: {}", address.green());

        Ok(Self {
            client: ForRunnerClient::connect(address).await?,
        })
    }
}

/// Runner-oriented impls
impl Client {
    pub async fn hello(&mut self) -> Result<PHelloReply> {
        let response = self.client
            .hello(PHelloRequest {})
            .await?;

        Ok(response.into_inner())
    }

    pub async fn ping(&mut self, runner_id: PRunnerId) -> Result<PPingReply> {
        let response = self.client
            .ping(PPingRequest { runner_id })
            .await?;

        Ok(response.into_inner())
    }

    pub async fn register(&mut self, name: String, secret: String) -> Result<PRegisterReply> {
        let response = self.client
            .register(PRegisterRequest { name, secret })
            .await?;

        Ok(response.into_inner())
    }
}

/// Assignment-oriented impls
impl Client {
    pub async fn request_assignment(&mut self, runner_id: PRunnerId) -> Result<PRequestAssignmentReply> {
        let response = self.client
            .request_assignment(PRequestAssignmentRequest { runner_id })
            .await?;

        Ok(response.into_inner())
    }
}

/// Experiment-oriented impls
impl Client {
    pub async fn add_experiment_report(
        &mut self,
        runner_id: PRunnerId,
        experiment_id: PExperimentId,
        report: PReport,
    ) -> Result<PAddExperimentReportReply> {
        let response = self.client
            .add_experiment_report(PAddExperimentReportRequest { runner_id, experiment_id, report: Some(report) })
            .await?;

        Ok(response.into_inner())
    }
}