use anyhow::*;

use lib_interop::proto::services::{PFindJobsReply, PFindJobsRequest};

use crate::system::ExperimentStore;

pub async fn find_jobs(
    experiment_store: &ExperimentStore,
    request: PFindJobsRequest,
) -> Result<PFindJobsReply> {
    let id = request.experiment_id.into();

    let jobs = experiment_store
        .find_one(id).await?
        .get_jobs().await;

    let jobs = jobs
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(PFindJobsReply { jobs })
}