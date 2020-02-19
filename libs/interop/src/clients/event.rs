use anyhow::*;
use tonic::transport::Channel;

use crate::connection::Connection;
use crate::models::{DEvent, DExperimentId, DRunnerId};
use crate::proto::services::events_client::EventsClient as EventsClientInner;
use crate::proto::services::PAddEventRequest;

#[derive(Clone)]
pub struct EventClient {
    inner: EventsClientInner<Channel>,
}

impl EventClient {
    crate fn new(conn: Connection) -> Self {
        Self {
            inner: EventsClientInner::with_interceptor(
                conn.channel,
                conn.interceptor,
            ),
        }
    }

    pub async fn add(&mut self, runner_id: DRunnerId, experiment_id: DExperimentId, event: DEvent) -> Result<()> {
        self.inner.add_event(PAddEventRequest {
            runner_id: runner_id.into(),
            experiment_id: experiment_id.into(),
            event: Some(event.into()),
        }).await?;

        Ok(())
    }
}