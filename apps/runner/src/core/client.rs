use colored::Colorize;
use log::*;
use tonic::transport::Channel;

use lib_interop::protocol::core::{PEvent, PExperimentId, PRunnerId};
use lib_interop::protocol::for_runner::*;
use lib_interop::protocol::for_runner::for_runner_client::ForRunnerClient;

use crate::Result;

pub use self::concerns::*;

mod concerns;

#[derive(Clone)]
pub struct Client {
    client: ForRunnerClient<Channel>,
}

impl Client {
    pub async fn connect(address: String) -> Result<Self> {
        info!("Connecting to controller at: {}", address.green());

        let client = ForRunnerClient::connect(address)
            .await?;

        info!("Connection acquired");

        Ok(Self { client })
    }
}

/// Controller-oriented impls
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

    pub async fn register(&mut self, name: String) -> Result<PRegisterReply> {
        let response = self.client
            .register(PRegisterRequest { name })
            .await?;

        Ok(response.into_inner())
    }
}

/// Assignment-oriented impls
impl Client {
    pub async fn get_assignment(&mut self, runner_id: PRunnerId) -> Result<PGetAssignmentReply> {
        let response = self.client
            .get_assignment(PGetAssignmentRequest { runner_id })
            .await?;

        Ok(response.into_inner())
    }
}

/// Event-oriented impls
impl Client {
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