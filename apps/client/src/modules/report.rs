use anyhow::*;

use lib_interop::convert;
use lib_interop::domain::{DExperimentId, DReport};
use lib_interop::proto::controller::PFindReportsRequest;

use crate::modules::app::AppContext;

pub struct ReportSearcher<'c> {
    ctxt: &'c mut AppContext,
}

impl<'c> ReportSearcher<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> Self {
        Self { ctxt }
    }

    pub async fn find_by_experiment_id(&mut self, experiment_id: DExperimentId) -> Result<Vec<DReport>> {
        let reports = self.ctxt
            .client()
            .await?
            .find_reports(PFindReportsRequest { experiment_id: experiment_id.into() })
            .await?
            .reports;

        Ok(convert!(reports as [_?]))
    }
}