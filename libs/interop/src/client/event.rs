use anyhow::*;

use crate::client::ControllerClient;
use crate::proto::controller::*;
use crate::proto::core::*;

impl ControllerClient {
    pub async fn add_event(
        &mut self,
        runner_id: PRunnerId,
        experiment_id: PExperimentId,
        event: PEvent,
    ) -> Result<PAddEventReply> {
        let response = self.client.add_event(PAddEventRequest {
            runner_id,
            experiment_id,
            event: Some(event),
        }).await?;

        Ok(response.into_inner())
    }
}
