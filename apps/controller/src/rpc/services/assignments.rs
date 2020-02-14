use tonic::{Request, Response, Interceptor, Status};

use lib_interop::proto::services::*;
use lib_interop::proto::services::assignments_server::{Assignments, AssignmentsServer};

use crate::system;

pub struct AssignmentsService {
    pub experiments: system::Experiments,
}

#[tonic::async_trait]
impl Assignments for AssignmentsService {
    async fn prepare_assignment(
        &self,
        request: Request<PPrepareAssignmentRequest>,
    ) -> Result<Response<PPrepareAssignmentReply>, Status> {
        unimplemented!()
    }
}