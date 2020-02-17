use tonic::{Request, Response, Status};

use lib_interop::proto::services::*;
use lib_interop::proto::services::assignments_server::Assignments;

use crate::system::ExperimentStore;

use super::transform_error;

mod prepare;

pub struct AssignmentsService {
    pub experiment_store: ExperimentStore,
}

#[tonic::async_trait]
impl Assignments for AssignmentsService {
    async fn prepare_assignment(
        &self,
        request: Request<PPrepareAssignmentRequest>,
    ) -> Result<Response<PPrepareAssignmentReply>, Status> {
        prepare::prepare_assignment(&self.experiment_store, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }
}