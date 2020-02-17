use std::convert::TryFrom;

use anyhow::*;

use lib_interop::domain::{DJob, DomainResult};
use lib_interop::proto::services::PFindJobsRequest;

use super::ExecutorActor;

impl ExecutorActor {
    pub async fn fetch_jobs(&mut self) -> Result<Vec<DJob>> {
        let jobs = self.session.conn
            .jobs()
            .find_jobs(PFindJobsRequest { experiment_id: self.experiment_id.into() })
            .await?
            .into_inner()
            .jobs;

        let jobs = jobs.into_iter()
            .map(DJob::try_from)
            .collect::<DomainResult<_>>()?;

        Ok(jobs)
    }
}