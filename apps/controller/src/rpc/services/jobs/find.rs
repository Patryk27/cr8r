use anyhow::*;

use lib_interop::proto::controller::{PFindReportsReply, PFindReportsRequest};

use crate::system::System;

pub async fn find_reports(
    system: &System,
    request: PFindReportsRequest,
) -> Result<PFindReportsReply> {
    let experiment_id = request.experiment_id.into();

    let experiment = system
        .experiments
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