use tonic::{Code, Request, Response, Status};

use lib_protocol::for_runner::*;
use lib_protocol::for_runner::server::ForRunner;

use crate::backend::System;

pub struct ForRunnerService {
    system: System,
}

impl ForRunnerService {
    pub fn new(system: System) -> Self {
        Self { system }
    }
}

// @todo validate runner's secret key
#[tonic::async_trait]
impl ForRunner for ForRunnerService {
    async fn hello(
        &self,
        _: Request<PHelloRequest>,
    ) -> Result<Response<PHelloReply>, Status> {
        Ok(Response::new(PHelloReply {
            version: "0.1.0".into(),
        }))
    }

    async fn register(
        &self,
        request: Request<PRegisterRequest>,
    ) -> Result<Response<PRegisterReply>, Status> {
        let request = request.into_inner();

        let id = self.system
            .register_runner(request.name)
            .await?;

        Ok(Response::new(PRegisterReply { id }))
    }

    async fn request_assignment(
        &self,
        request: Request<PRequestAssignmentRequest>,
    ) -> Result<Response<PRequestAssignmentReply>, Status> {
        let request = request.into_inner();

        let assignment = self.system
            .request_assignment(request.runner_id)
            .await?;

        Ok(Response::new(PRequestAssignmentReply { assignment }))
    }

    async fn add_experiment_report(
        &self,
        request: Request<PAddExperimentReportRequest>,
    ) -> Result<Response<PAddExperimentReportReply>, Status> {
        let request = request.into_inner();

        if let Some(report) = request.report {
            self.system
                .find_experiment(request.experiment_id).await?
                .add_report(request.runner_id, report).await?;

            Ok(Response::new(PAddExperimentReportReply {}))
        } else {
            Err(Status::new(Code::Internal, "No report has been provided"))
        }
    }
}