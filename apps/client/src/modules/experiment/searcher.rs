use std::convert::TryInto;

use anyhow::*;

use lib_interop::domain::{DExperiment, DExperimentId};
use lib_interop::proto::services::PFindExperimentsRequest;

use crate::modules::app::AppContext;

pub struct ExperimentSearcher<'c> {
    ctxt: &'c mut AppContext,
}

impl<'c> ExperimentSearcher<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> Self {
        Self { ctxt }
    }

    pub async fn find_by_id(&mut self, id: DExperimentId) -> Result<Option<DExperiment>> {
        let mut experiments = self.ctxt
            .experiments()
            .await?
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
}