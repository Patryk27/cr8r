use tonic::{Request, Response, Status, Streaming};

use lib_interop::proto::services::*;
use lib_interop::proto::services::jobs_server::Jobs;

use crate::system;

pub struct JobsService {
    pub experiments: system::Experiments,
}

#[tonic::async_trait]
impl Jobs for JobsService {
    async fn find_jobs(
        &self,
        request: Request<PFindJobsRequest>,
    ) -> Result<Response<PFindJobsReply>, Status> {
        unimplemented!()
    }
}
