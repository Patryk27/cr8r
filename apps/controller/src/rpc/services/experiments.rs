use tokio::stream::Stream;
use tonic::{Request, Response, Status};

use lib_interop::proto::models::PReport;
use lib_interop::proto::services::*;
use lib_interop::proto::services::experiments_server::Experiments;

use crate::system;

use super::transform_error;

mod create;
mod delete;
mod find;
mod stop;
mod watch;

pub struct ExperimentsService {
    pub experiments: system::Experiments,
}

#[tonic::async_trait]
impl Experiments for ExperimentsService {
    async fn create_experiment(
        &self,
        request: Request<PCreateExperimentRequest>,
    ) -> Result<Response<PCreateExperimentReply>, Status> {
        create::create_experiment(&self.experiments, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn delete_experiment(
        &self,
        request: Request<PDeleteExperimentRequest>,
    ) -> Result<Response<PDeleteExperimentReply>, Status> {
        delete::delete_experiment(&self.experiments, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn find_experiments(
        &self,
        request: Request<PFindExperimentsRequest>,
    ) -> Result<Response<PFindExperimentsReply>, Status> {
        find::find_experiments(&self.experiments, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn stop_experiment(
        &self,
        request: Request<PStopExperimentRequest>,
    ) -> Result<Response<PStopExperimentReply>, Status> {
        stop::stop_experiment(&self.experiments, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    type WatchExperimentStream = impl Stream<Item=Result<PReport, Status>>;

    async fn watch_experiment(
        &self,
        request: Request<PWatchExperimentRequest>,
    ) -> Result<Response<Self::WatchExperimentStream>, Status> {
        watch::watch_experiment(&self.experiments, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }
}