use anyhow::*;

use crate::client::ControllerClient;
use crate::proto::controller::*;
use crate::proto::core::*;

impl ControllerClient {
    pub async fn get_assignment(&mut self, runner_id: PRunnerId) -> Result<PGetAssignmentReply> {
        let response = self.client
            .get_assignment(PGetAssignmentRequest { runner_id })
            .await?;

        Ok(response.into_inner())
    }
}
