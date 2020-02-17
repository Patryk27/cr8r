use tonic::{Request, Response, Status};

use lib_interop::proto::services::*;
use lib_interop::proto::services::events_server::Events;

use crate::system::ExperimentStore;

use super::transform_error;

mod add;

pub struct EventsService {
    pub experiment_store: ExperimentStore,
}

#[tonic::async_trait]
impl Events for EventsService {
    async fn add_event(
        &self,
        request: Request<PAddEventRequest>,
    ) -> Result<Response<PAddEventReply>, Status> {
        add::add_event(&self.experiment_store, request.into_inner())
            .await
            .map(Response::new)
            .map_err(transform_error)
    }
}