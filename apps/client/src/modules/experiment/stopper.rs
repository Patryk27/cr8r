use anyhow::*;

use lib_interop::proto::core::PExperimentId;

use crate::modules::app::AppContext;

pub struct ExperimentStopper<'c> {
    ctxt: &'c mut AppContext,
}

impl<'c> ExperimentStopper<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> Self {
        Self { ctxt }
    }

    pub async fn stop(&mut self, id: PExperimentId) -> Result<()> {
        self.ctxt
            .client()
            .await?
            .stop_experiment(id)
            .await?;

        Ok(())
    }
}