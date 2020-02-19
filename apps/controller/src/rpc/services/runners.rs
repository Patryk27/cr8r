use tonic::{Request, Response, Status};

use lib_interop::proto::services::*;
use lib_interop::proto::services::runners_server::Runners;

use crate::system::RunnerStore;

use super::transform_error;

mod find;
mod register;
mod sync_heartbeat;

pub struct RunnersService {
    pub runner_store: RunnerStore,
}

#[tonic::async_trait]
impl Runners for RunnersService {
    async fn find_runners(
        &self,
        _: Request<PFindRunnersRequest>,
    ) -> Result<Response<PFindRunnersReply>, Status> {
        find::find_runners(&self.runner_store).await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn register_runner(
        &self,
        request: Request<PRegisterRunnerRequest>,
    ) -> Result<Response<PRegisterRunnerReply>, Status> {
        register::register_runner(&self.runner_store, request.into_inner()).await
            .map(Response::new)
            .map_err(transform_error)
    }

    async fn sync_runner_heartbeat(
        &self,
        request: Request<PSyncRunnerHeartbeatRequest>,
    ) -> Result<Response<PSyncRunnerHeartbeatReply>, Status> {
        sync_heartbeat::sync_heartbeat(&self.runner_store, request.into_inner()).await
            .map(Response::new)
            .map_err(transform_error)
    }
}