use tonic::{Code, Request, Response, Status};

use lib_protocol::client::*;
use lib_protocol::client::server::Client;
use lib_protocol::core::ExperimentDefinition;

use crate::backend::System;

pub struct ClientService {
    system: System,
}

impl ClientService {
    pub fn new(system: System) -> Self {
        Self { system }
    }
}

#[tonic::async_trait]
impl Client for ClientService {
    async fn hello(&self, _: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            version: "0.1.0".into(),
            uptime: 0, // @todo
        }))
    }

    async fn abort_experiment(
        &self,
        request: Request<AbortExperimentRequest>,
    ) -> Result<Response<AbortExperimentReply>, Status> {
        let request = request.into_inner();

        unimplemented!()

//        self.system
//            .find_experiment(request.id).await?
//            .abort();
//
//        Ok(Response::new(AbortExperimentReply {}))
    }

    async fn launch_experiment(
        &self,
        request: Request<LaunchExperimentRequest>,
    ) -> Result<Response<LaunchExperimentReply>, Status> {
        let request = request.into_inner();

        if let Some(ExperimentDefinition { experiment_definition_inner: Some(experiment) }) = request.experiment {
            let id = self.system.launch_experiment(experiment).await?;

            Ok(Response::new(LaunchExperimentReply {
                id,
                position_in_queue: 1, // @todo
            }))
        } else {
            Err(Status::new(Code::Internal, "No experiment to launch has been provided"))
        }
    }
}