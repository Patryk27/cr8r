use anyhow::*;
use tonic::transport::Channel;

use crate::connection::Connection;
use crate::conv;
use crate::models::{DExperimentId, DJob};
use crate::proto::services::*;
use crate::proto::services::jobs_client::JobsClient as JobsClientInner;

#[derive(Clone)]
pub struct JobClient {
    inner: JobsClientInner<Channel>,
}

impl JobClient {
    crate fn new(conn: Connection) -> Self {
        Self {
            inner: JobsClientInner::with_interceptor(
                conn.channel,
                conn.interceptor,
            ),
        }
    }

    pub async fn find_many(&mut self, experiment_id: DExperimentId) -> Result<Vec<DJob>> {
        let jobs = self.inner
            .find_jobs(PFindJobsRequest { experiment_id: experiment_id.into() })
            .await?
            .into_inner()
            .jobs;

        Ok(conv!(jobs as [_?]))
    }
}