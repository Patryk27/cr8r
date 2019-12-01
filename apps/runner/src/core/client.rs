use colored::Colorize;
use log::*;
use tonic::transport::Channel;

use lib_protocol::core::RunnerId;
use lib_protocol::runner::*;
use lib_protocol::runner::client::RunnerClient;

use crate::Result;

#[derive(Clone)]
pub struct Client {
    client: RunnerClient<Channel>,
}

impl Client {
    pub async fn connect(address: String) -> Result<Self> {
        info!("Connecting to controller at: {}", address.green());

        Ok(Self {
            client: RunnerClient::connect(address).await?,
        })
    }

    pub async fn hello(&mut self) -> Result<HelloReply> {
        let response = self.client.hello(HelloRequest {
            //
        }).await?;

        Ok(response.into_inner())
    }

    pub async fn ping(&mut self, runner_id: RunnerId) -> Result<PingReply> {
        let response = self.client.ping(PingRequest {
            runner_id,
        }).await?;

        Ok(response.into_inner())
    }

    pub async fn register(&mut self, name: String, secret: String) -> Result<RegisterReply> {
        let response = self.client.register(RegisterRequest {
            name,
            secret,
        }).await?;

        Ok(response.into_inner())
    }

    pub async fn request_assignment(&mut self, runner_id: RunnerId) -> Result<RequestAssignmentReply> {
        let response = self.client.request_assignment(RequestAssignmentRequest {
            runner_id,
        }).await?;

        Ok(response.into_inner())
    }
}