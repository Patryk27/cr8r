use tonic::{Request, Response, Status, Streaming};

use lib_interop::proto::models::PReport;
use lib_interop::proto::services::*;
use lib_interop::proto::services::experiments_server::Experiments;

use crate::system;

pub struct ExperimentsService {
    pub experiments: system::Experiments,
}

#[tonic::async_trait]
impl Experiments for ExperimentsService {
    async fn create_experiment(
        &self,
        request: Request<PCreateExperimentRequest>,
    ) -> Result<Response<PCreateExperimentReply>, Status> {
        unimplemented!()
    }

    async fn delete_experiment(
        &self,
        request: Request<PDeleteExperimentRequest>,
    ) -> Result<Response<PDeleteExperimentReply>, Status> {
        unimplemented!()
    }

    async fn find_experiments(
        &self,
        request: Request<PFindExperimentsRequest>,
    ) -> Result<Response<PFindExperimentsReply>, Status> {
        unimplemented!()
    }

    async fn stop_experiment(
        &self,
        request: Request<PStopExperimentRequest>,
    ) -> Result<Response<PStopExperimentReply>, Status> {
        unimplemented!()
    }

    type WatchExperimentStream = ();

    async fn watch_experiment(
        &self,
        request: Request<PWatchExperimentRequest>,
    ) -> Result<Response<Self::WatchExperimentStream>, Status> {
        unimplemented!()
    }
}