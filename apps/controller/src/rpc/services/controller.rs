use tonic::{Request, Response, Status};

use lib_interop::proto::services::*;
use lib_interop::proto::services::controller_server::Controller;

mod howdy;

pub struct ControllerService;

#[tonic::async_trait]
impl Controller for ControllerService {
    async fn howdy(
        &self,
        _: Request<PHowdyRequest>,
    ) -> Result<Response<PHowdyReply>, Status> {
        Ok(Response::new(
            howdy::howdy()
        ))
    }
}