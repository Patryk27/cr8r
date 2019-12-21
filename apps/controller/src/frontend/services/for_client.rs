use tokio::stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};

use lib_interop::protocol::core::PReport;
use lib_interop::protocol::for_client::*;
use lib_interop::protocol::for_client::for_client_server::ForClient;

use crate::backend::System;

pub struct ForClientService {
    system: System,
}

impl ForClientService {
    pub fn new(system: System) -> Self {
        Self { system }
    }
}

mod create_experiment;
mod find_reports;
mod find_experiments;
mod find_runners;
mod hello;
mod watch_experiment;

// @todo validate client's secret key
#[tonic::async_trait]
impl ForClient for ForClientService {
    async fn hello(
        &self,
        _: Request<PHelloRequest>,
    ) -> Result<Response<PHelloReply>, Status> {
        Ok(Response::new(
            hello::hello()
        ))
    }

    async fn create_experiment(
        &self,
        request: Request<PCreateExperimentRequest>,
    ) -> Result<Response<PCreateExperimentReply>, Status> {
        create_experiment::create_experiment(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(Status::internal)
    }

    type WatchExperimentStream = impl Stream<Item=Result<PReport, Status>>;

    async fn watch_experiment(
        &self,
        request: Request<PWatchExperimentRequest>,
    ) -> Result<Response<Self::WatchExperimentStream>, Status> {
        watch_experiment::watch_experiment(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(Status::internal)
    }

    async fn find_experiments(
        &self,
        request: Request<PFindExperimentsRequest>,
    ) -> Result<Response<PFindExperimentsReply>, Status> {
        let reply = find_experiments::find_experiments(&self.system, request.into_inner())
            .await;

        Ok(Response::new(reply))
    }

    async fn find_reports(
        &self,
        request: Request<PFindReportsRequest>,
    ) -> Result<Response<PFindReportsReply>, Status> {
        find_reports::find_reports(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(Status::internal)
    }

    async fn find_runners(
        &self,
        _: Request<PFindRunnersRequest>,
    ) -> Result<Response<PFindRunnersReply>, Status> {
        let reply = find_runners::find_runners(&self.system)
            .await;

        Ok(Response::new(reply))
    }
}