use tonic::{Request, Response, Status};

use lib_interop::proto::services::*;
use lib_interop::proto::services::jobs_server::Jobs;

use crate::system;

use super::transform_error;

mod find;

pub struct JobsService {
    pub experiments: system::Experiments,
}

#[tonic::async_trait]
impl Jobs for JobsService {
    async fn find_jobs(
        &self,
        request: Request<PFindJobsRequest>,
    ) -> Result<Response<PFindJobsReply>, Status> {
        find::find_jobs(&self.experiments, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }
}
