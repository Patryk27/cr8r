use anyhow::*;

use crate::client::ControllerClient;
use crate::proto::controller::*;

impl ControllerClient {
    pub async fn find_reports(&mut self, request: PFindReportsRequest) -> Result<PFindReportsReply> {
        let response = self.client.find_reports(request)
            .await?;

        Ok(response.into_inner())
    }
}
