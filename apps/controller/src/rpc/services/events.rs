use tonic::{Request, Response, Status, Streaming};

use lib_interop::proto::services::*;
use lib_interop::proto::services::events_server::Events;

use crate::system;

pub struct EventsService {
    pub experiments: system::Experiments,
}

#[tonic::async_trait]
impl Events for EventsService {
    async fn add_event(
        &self,
        request: Request<PAddEventRequest>,
    ) -> Result<Response<PAddEventReply>, Status> {
        unimplemented!()
    }
}