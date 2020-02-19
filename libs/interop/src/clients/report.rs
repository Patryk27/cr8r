use anyhow::*;
use tonic::transport::Channel;

use crate::connection::Connection;
use crate::conv;
use crate::models::{DExperimentId, DReport};
use crate::proto::services::*;
use crate::proto::services::reports_client::ReportsClient as ReportsClientInner;

#[derive(Clone)]
pub struct ReportClient {
    inner: ReportsClientInner<Channel>,
}

impl ReportClient {
    crate fn new(conn: Connection) -> Self {
        Self {
            inner: ReportsClientInner::with_interceptor(
                conn.channel,
                conn.interceptor,
            ),
        }
    }

    pub async fn find_many(&mut self, experiment_id: DExperimentId) -> Result<Vec<DReport>> {
        let reports = self.inner
            .find_reports(PFindReportsRequest { experiment_id: experiment_id.into() }).await?
            .into_inner()
            .reports;

        Ok(conv!(reports as [_?]))
    }
}
