use std::convert::TryInto;

use anyhow::*;

use lib_interop::domain::{DExperiment, DExperimentId};
use lib_interop::proto::controller::PFindExperimentsRequest;

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
            .client()
            .await?
            .find_experiments(PFindExperimentsRequest { id: id.into() })
            .await?
            .experiments;

        let experiment = experiments
            .drain(..)
            .next()
            .map(|experiment| -> Result<DExperiment> { Ok(experiment.try_into()?) })
            .transpose();

        experiment
    }
}