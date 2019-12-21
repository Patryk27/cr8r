use lib_interop::protocol::for_client::{PFindExperimentReportsReply, PFindExperimentReportsRequest};

use crate::backend::{Result, System};

pub async fn find_experiment_reports(
    system: &System,
    request: PFindExperimentReportsRequest,
) -> Result<PFindExperimentReportsReply> {
    let reports = system
        .find_experiment(request.filter_experiment_id.into()).await?
        .get_reports().await
        .into_iter()
        .map(|report| (&*report).into())
        .collect();

    Ok(PFindExperimentReportsReply { reports })
}