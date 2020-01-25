use anyhow::*;

use lib_interop::proto::controller::{PFindReportsReply, PFindReportsRequest};

use crate::backend::System;

pub async fn find_reports(
    system: &System,
    request: PFindReportsRequest,
) -> Result<PFindReportsReply> {
    let reports = system
        .find_experiment(request.experiment_id.into())
        .await?
        .get_reports()
        .await
        .into_iter()
        .map(|report| (&*report).into())
        .collect();

    Ok(PFindReportsReply { reports })
}