use anyhow::*;

use lib_interop::proto::services::{PFindReportsReply, PFindReportsRequest};

use crate::system::Experiments;

pub async fn find_reports(
    experiments: &Experiments,
    request: PFindReportsRequest,
) -> Result<PFindReportsReply> {
    let experiment_id = request.experiment_id.into();

    let experiment = experiments
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