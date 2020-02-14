use anyhow::*;

use crate::client::ControllerClient;
use crate::proto::controller::*;
use crate::proto::core::*;

impl ControllerClient {
    pub async fn prepare_assignment(&mut self, runner_id: PRunnerId) -> Result<PPrepareAssignmentReply> {
        let response = self.client
            .prepare_assignment(PPrepareAssignmentRequest { runner_id })
            .await?;

        Ok(response.into_inner())
    }
}
