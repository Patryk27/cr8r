use tonic::{Request, Response, Status};

use lib_interop::proto::services::*;
use lib_interop::proto::services::runners_server::Runners;

use crate::system;

use super::transform_error;

mod find;
mod register;

pub struct RunnersService {
    pub runners: system::Runners,
}

#[tonic::async_trait]
impl Runners for RunnersService {
    async fn find_runners(
        &self,
        _: Request<PFindRunnersRequest>,
    ) -> Result<Response<PFindRunnersReply>, Status> {
        find::find_runners(&self.runners)
            .await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn register_runner(
        &self,
        request: Request<PRegisterRunnerRequest>,
    ) -> Result<Response<PRegisterRunnerReply>, Status> {
        register::register_runner(&self.runners, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }
}