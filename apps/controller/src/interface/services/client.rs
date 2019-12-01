use tonic::{Code, Request, Response, Status};

use lib_protocol::client::*;
use lib_protocol::client::server::Client;
use lib_protocol::core::ExperimentDefinition;

use crate::system::System;

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

        if let Ok(id) = request.id.parse() {
            self.system.abort_experiment(id)
                .await
                .map(|_| Response::new(AbortExperimentReply {}))
                .map_err(|err| Status::new(Code::Internal, err))
        } else {
            Err(Status::new(Code::Internal, "Experiment\'s id has an invalid format"))
        }
    }

    async fn launch_experiment(
        &self,
        request: Request<LaunchExperimentRequest>,
    ) -> Result<Response<LaunchExperimentReply>, Status> {
        let request = request.into_inner();

        if let Some(ExperimentDefinition { experiment_definition_inner: Some(experiment) }) = request.experiment {
            unimplemented!()

//            self.system.launch_experiment(experiment)
//                .await
//                .map(|(id, position_in_queue)| Response::new(LaunchExperimentReply {
//                    id: id.to_hyphenated().to_string(),
//                    position_in_queue,
//                }))
//                .map_err(|err| Status::new(Code::Internal, err))
        } else {
            Err(Status::new(Code::Internal, "No experiment has been provided"))
        }
    }
}