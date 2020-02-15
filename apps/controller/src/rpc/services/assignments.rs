use tonic::{Request, Response, Status};

use lib_interop::proto::services::*;
use lib_interop::proto::services::assignments_server::Assignments;

use crate::system;

use super::transform_error;

mod prepare;

pub struct AssignmentsService {
    pub experiments: system::Experiments,
}

#[tonic::async_trait]
impl Assignments for AssignmentsService {
    async fn prepare_assignment(
        &self,
        request: Request<PPrepareAssignmentRequest>,
    ) -> Result<Response<PPrepareAssignmentReply>, Status> {
        prepare::prepare_assignment(&self.experiments, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }
}