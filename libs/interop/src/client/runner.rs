use anyhow::*;

use crate::client::ControllerClient;
use crate::proto::controller::*;
use crate::proto::core::*;

impl ControllerClient {
    pub async fn find_runners(&mut self, request: PFindRunnersRequest) -> Result<PFindRunnersReply> {
        let response = self.client.find_runners(request)
            .await?;

        Ok(response.into_inner())
    }

    pub async fn register_runner(&mut self, name: PRunnerName) -> Result<PRegisterRunnerReply> {
        let response = self.client
            .register_runner(PRegisterRunnerRequest { name })
            .await?;

        Ok(response.into_inner())
    }
}
