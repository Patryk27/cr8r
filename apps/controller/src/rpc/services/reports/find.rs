use anyhow::*;

use lib_interop::proto::services::{PFindReportsReply, PFindReportsRequest};

use crate::system::ExperimentStore;

pub async fn find_reports(
    experiment_store: &ExperimentStore,
    request: PFindReportsRequest,
) -> Result<PFindReportsReply> {
    let experiment_id = request.experiment_id.into();

    let experiment = experiment_store
        .find_one(experiment_id)
        .await?;

    let reports = experiment
        .get_reports()
        .await;

    let reports = reports
        .into_iter()
        .map(|report| (&*report).into())
        .collect();

    Ok(PFindReportsReply { reports })
}