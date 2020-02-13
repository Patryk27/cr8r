use anyhow::*;
use tonic::Streaming;

use crate::client::ControllerClient;
use crate::proto::controller::*;
use crate::proto::core::*;

impl ControllerClient {
    pub async fn create_experiment(&mut self, definition: PDefinition) -> Result<PCreateExperimentReply> {
        let response = self.client
            .create_experiment(PCreateExperimentRequest { definition: Some(definition) })
            .await?;

        Ok(response.into_inner())
    }

    pub async fn delete_experiment(&mut self, id: PExperimentId) -> Result<PDeleteExperimentReply> {
        let response = self.client
            .delete_experiment(PDeleteExperimentRequest { id })
            .await?;

        Ok(response.into_inner())
    }

    pub async fn find_experiments(&mut self, request: PFindExperimentsRequest) -> Result<PFindExperimentsReply> {
        let response = self.client
            .find_experiments(request)
            .await?;

        Ok(response.into_inner())
    }

    pub async fn stop_experiment(&mut self, id: PExperimentId) -> Result<PStopExperimentReply> {
        let response = self.client
            .stop_experiment(PStopExperimentRequest { id })
            .await?;

        Ok(response.into_inner())
    }

    pub async fn watch_experiment(&mut self, id: PExperimentId) -> Result<Streaming<PReport>> {
        let response = self.client
            .watch_experiment(PWatchExperimentRequest { id })
            .await?;

        Ok(response.into_inner())
    }
}
