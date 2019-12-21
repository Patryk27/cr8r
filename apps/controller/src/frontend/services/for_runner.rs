use tonic::{Request, Response, Status};

use lib_interop::protocol::for_runner::*;
use lib_interop::protocol::for_runner::for_runner_server::ForRunner;

use crate::backend::System;

pub struct ForRunnerService {
    system: System,
}

impl ForRunnerService {
    pub fn new(system: System) -> Self {
        Self { system }
    }
}

mod add_event;
mod get_assignment;
mod hello;
mod register;

// @todo validate runner's secret key
#[tonic::async_trait]
impl ForRunner for ForRunnerService {
    async fn hello(
        &self,
        _: Request<PHelloRequest>,
    ) -> Result<Response<PHelloReply>, Status> {
        Ok(Response::new(
            hello::hello()
        ))
    }

    async fn register(
        &self,
        request: Request<PRegisterRequest>,
    ) -> Result<Response<PRegisterReply>, Status> {
        register::register(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(Status::internal)
    }

    async fn get_assignment(
        &self,
        request: Request<PGetAssignmentRequest>,
    ) -> Result<Response<PGetAssignmentReply>, Status> {
        get_assignment::get_assignment(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(Status::internal)
    }

    async fn add_event(
        &self,
        request: Request<PAddEventRequest>,
    ) -> Result<Response<PAddEventReply>, Status> {
        add_event::add_event(&self.system, request.into_inner())
            .await
            .map(Response::new)
            .map_err(Status::internal)
    }
}