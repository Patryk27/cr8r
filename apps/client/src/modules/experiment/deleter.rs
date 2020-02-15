use anyhow::*;

use lib_interop::proto::models::PExperimentId;
use lib_interop::proto::services::PDeleteExperimentRequest;

use crate::modules::app::AppContext;

pub struct ExperimentDeleter<'c> {
    ctxt: &'c mut AppContext,
}

impl<'c> ExperimentDeleter<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> Self {
        Self { ctxt }
    }

    pub async fn delete(&mut self, id: PExperimentId) -> Result<()> {
        self.ctxt
            .experiments()
            .await?
            .delete_experiment(PDeleteExperimentRequest { id })
            .await?;

        Ok(())
    }
}