use anyhow::*;

use lib_interop::proto::core::PExperimentId;

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
            .client()
            .await?
            .delete_experiment(id)
            .await?;

        Ok(())
    }
}