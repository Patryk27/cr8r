use tonic::{Request, Response, Status};

use lib_protocol::controller::{HelloReply, HelloRequest};
use lib_protocol::controller::server::Controller;

pub struct ControllerService;

impl ControllerService {
    pub fn new() -> Self {
        Self
    }
}

#[tonic::async_trait]
impl Controller for ControllerService {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            version: "0.1.0".into(),
        };

        Ok(Response::new(reply))
    }
}