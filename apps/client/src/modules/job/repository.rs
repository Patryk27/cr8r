use anyhow::*;
use tonic::transport::Channel;

use lib_interop::conv;
use lib_interop::domain::{DExperimentId, DJob};
use lib_interop::proto::services::jobs_client::JobsClient;
use lib_interop::proto::services::PFindJobsRequest;

use crate::modules::app::AppContext;

pub struct JobRepository {
    jobs_client: JobsClient<Channel>,
}

impl JobRepository {
    pub async fn new(ctxt: &mut AppContext) -> Result<Self> {
        Ok(Self {
            jobs_client: ctxt.jobs().await?,
        })
    }

    pub async fn find(&mut self, experiment_id: DExperimentId) -> Result<Vec<DJob>> {
        let jobs = self.jobs_client
            .find_jobs(PFindJobsRequest { experiment_id: experiment_id.into() })
            .await?
            .into_inner()
            .jobs;

        Ok(conv!(jobs as [_?]))
    }
}