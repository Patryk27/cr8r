use tonic::{Code, Request, Response, Status};

use lib_protocol::runner::*;
use lib_protocol::runner::server::Runner;

use crate::system::System;

pub struct RunnerService {
    system: System,
}

impl RunnerService {
    pub fn new(system: System) -> Self {
        Self { system }
    }
}

#[tonic::async_trait]
impl Runner for RunnerService {
    async fn hello(&self, _: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            version: "0.1.0".into(),
        }))
    }

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterReply>, Status> {
        let request = request.into_inner();

        self.system.register_runner(request.name, request.secret)
            .await
            .map(|token| Response::new(RegisterReply {
                token: token.to_hyphenated().to_string(),
            }))
            .map_err(|err| Status::new(Code::Internal, err))
    }

    async fn request_experiment(
        &self,
        request: Request<RequestExperimentRequest>,
    ) -> Result<Response<RequestExperimentReply>, Status> {
        let request = request.into_inner();

        if let Ok(token) = request.token.parse() {
            self.system.request_experiment(token)
                .await
                .map(|assignment| Response::new(RequestExperimentReply {
                    assignment,
                }))
                .map_err(|err| Status::new(Code::Internal, err))
        } else {
            Err(Status::new(Code::Internal, "Runner\'s id has an invalid format"))
        }
    }
}