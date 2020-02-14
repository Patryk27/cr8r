use tonic::{Request, Response, Status, Streaming};

use lib_interop::proto::services::*;
use lib_interop::proto::services::controller_server::Controller;

pub struct ControllerService;

#[tonic::async_trait]
impl Controller for ControllerService {
    async fn howdy(
        &self,
        request: Request<PHowdyRequest>,
    ) -> Result<Response<PHowdyReply>, Status> {
        unimplemented!()
    }
}