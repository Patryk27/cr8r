use std::convert::TryInto;

use anyhow::*;
use tonic::transport::Channel;

use lib_interop::domain::{DExperiment, DExperimentId};
use lib_interop::proto::models::PExperimentId;
use lib_interop::proto::services::{PDeleteExperimentRequest, PFindExperimentsRequest, PStopExperimentRequest};
use lib_interop::proto::services::experiments_client::ExperimentsClient;

use crate::modules::app::AppContext;

pub struct ExperimentRepository {
    experiments_client: ExperimentsClient<Channel>,
}

impl ExperimentRepository {
    pub async fn new(ctxt: &mut AppContext) -> Result<Self> {
        Ok(Self {
            experiments_client: ctxt.experiments().await?,
        })
    }

    pub async fn find_one(&mut self, id: DExperimentId) -> Result<Option<DExperiment>> {
        let mut experiments = self.experiments_client
            .find_experiments(PFindExperimentsRequest { id: id.into() })
            .await?
            .into_inner()
            .experiments;

        let experiments = experiments
            .drain(..)
            .next()
            .map(|experiment| -> Result<DExperiment> { Ok(experiment.try_into()?) })
            .transpose();

        experiments
    }

    pub async fn stop(&mut self, id: PExperimentId) -> Result<()> {
        self.experiments_client
            .stop_experiment(PStopExperimentRequest { id })
            .await?;

        Ok(())
    }

    pub async fn delete(&mut self, id: PExperimentId) -> Result<()> {
        self.experiments_client
            .delete_experiment(PDeleteExperimentRequest { id })
            .await?;

        Ok(())
    }
}