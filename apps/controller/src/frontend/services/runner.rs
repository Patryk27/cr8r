use tonic::{Code, Request, Response, Status};

use lib_protocol::runner::*;
use lib_protocol::runner::server::Runner;

use crate::backend::System;

pub struct RunnerService {
    system: System,
}

impl RunnerService {
    pub fn new(system: System) -> Self {
        Self { system }
    }
}

// @todo validate runner's secret key
#[tonic::async_trait]
impl Runner for RunnerService {
    async fn hello(&self, _: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            version: "0.1.0".into(),
        }))
    }

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterReply>, Status> {
        let request = request.into_inner();

        let id = self.system
            .register_runner(request.name)
            .await?;

        Ok(Response::new(RegisterReply { id }))
    }

    async fn request_assignment(
        &self,
        request: Request<RequestAssignmentRequest>,
    ) -> Result<Response<RequestAssignmentReply>, Status> {
        let request = request.into_inner();

        let assignment = self.system
            .request_assignment(request.runner_id)
            .await?;

        Ok(Response::new(RequestAssignmentReply { assignment }))
    }

    async fn report_experiment(
        &self,
        request: Request<ReportExperimentRequest>,
    ) -> Result<Response<ReportExperimentReply>, Status> {
        let request = request.into_inner();

        if let Some(report) = request.report {
            self.system
                .find_experiment(request.experiment_id).await?
                .report(request.runner_id, report).await?;

            Ok(Response::new(ReportExperimentReply {
                //
            }))
        } else {
            Err(Status::new(Code::Internal, "No report has been provided"))
        }
    }
}