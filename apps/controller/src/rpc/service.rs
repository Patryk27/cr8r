use anyhow::*;
use tokio::stream::Stream;
use tonic::{Request, Response, Status, Streaming};

use lib_core_channel::URx;
use lib_interop::proto::controller::*;
use lib_interop::proto::controller::controller_server::Controller;
use lib_interop::proto::core::PReport;

use crate::system::System;

use self::{
    assignment::*,
    attachment::*,
    event::*,
    experiment::*,
    howdy::*,
    report::*,
    runner::*,
};

mod assignment;
mod attachment;
mod event;
mod experiment;
mod howdy;
mod report;
mod runner;

pub struct ControllerService {
    system: System,
}

impl ControllerService {
    pub fn new(system: System) -> Self {
        Self { system }
    }
}

#[tonic::async_trait]
impl Controller for ControllerService {
    async fn howdy(
        &self,
        _: Request<PHowdyRequest>,
    ) -> Result<Response<PHowdyReply>, Status> {
        Ok(Response::new(
            howdy()
        ))
    }

    async fn prepare_assignment(
        &self,
        request: Request<PPrepareAssignmentRequest>,
    ) -> Result<Response<PPrepareAssignmentReply>, Status> {
        prepare_assignment(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    type DownloadAttachmentStream = URx<Result<PDownloadAttachmentReply, Status>>;

    async fn download_attachment(
        &self,
        request: Request<PDownloadAttachmentRequest>,
    ) -> Result<Response<Self::DownloadAttachmentStream>, Status> {
        unimplemented!()
    }

    async fn upload_attachment(
        &self,
        request: Request<Streaming<PUploadAttachmentRequest>>,
    ) -> Result<Response<PUploadAttachmentReply>, Status> {
        upload_attachment(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn create_experiment(
        &self,
        request: Request<PCreateExperimentRequest>,
    ) -> Result<Response<PCreateExperimentReply>, Status> {
        create_experiment(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn delete_experiment(
        &self,
        request: Request<PDeleteExperimentRequest>,
    ) -> Result<Response<PDeleteExperimentReply>, Status> {
        delete_experiment(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn find_experiments(
        &self,
        request: Request<PFindExperimentsRequest>,
    ) -> Result<Response<PFindExperimentsReply>, Status> {
        find_experiments(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn stop_experiment(
        &self,
        request: Request<PStopExperimentRequest>,
    ) -> Result<Response<PStopExperimentReply>, Status> {
        stop_experiment(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    type WatchExperimentStream = impl Stream<Item=Result<PReport, Status>>;

    async fn watch_experiment(
        &self,
        request: Request<PWatchExperimentRequest>,
    ) -> Result<Response<Self::WatchExperimentStream>, Status> {
        watch_experiment(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn add_event(
        &self,
        request: Request<PAddEventRequest>,
    ) -> Result<Response<PAddEventReply>, Status> {
        add_event(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn find_reports(
        &self,
        request: Request<PFindReportsRequest>,
    ) -> Result<Response<PFindReportsReply>, Status> {
        find_reports(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn find_runners(
        &self,
        _: Request<PFindRunnersRequest>,
    ) -> Result<Response<PFindRunnersReply>, Status> {
        let reply = find_runners(&self.system)
            .await;

        Ok(Response::new(reply))
    }

    async fn register_runner(
        &self,
        request: Request<PRegisterRunnerRequest>,
    ) -> Result<Response<PRegisterRunnerReply>, Status> {
        register_runner(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }
}

fn transform_error(err: Error) -> Status {
    // @todo we could return more contextual status codes
    Status::unknown(err.to_string())
}