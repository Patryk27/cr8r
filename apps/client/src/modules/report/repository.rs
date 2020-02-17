use anyhow::*;
use tonic::transport::Channel;

use lib_interop::conv;
use lib_interop::domain::{DExperimentId, DReport};
use lib_interop::proto::services::PFindReportsRequest;
use lib_interop::proto::services::reports_client::ReportsClient;

use crate::modules::app::AppContext;

pub struct ReportRepository {
    reports_client: ReportsClient<Channel>,
}

impl ReportRepository {
    pub async fn new(ctxt: &mut AppContext) -> Result<Self> {
        Ok(Self {
            reports_client: ctxt.reports().await?,
        })
    }

    pub async fn find(&mut self, experiment_id: DExperimentId) -> Result<Vec<DReport>> {
        let reports = self.reports_client
            .find_reports(PFindReportsRequest { experiment_id: experiment_id.into() })
            .await?
            .into_inner()
            .reports;

        Ok(conv!(reports as [_?]))
    }
}
