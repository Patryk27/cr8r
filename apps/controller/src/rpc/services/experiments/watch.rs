use std::result;

use anyhow::*;
use tokio::stream::{Stream, StreamExt};
use tonic::Status;

use lib_interop::proto::models::PReport;
use lib_interop::proto::services::PWatchExperimentRequest;

use crate::system::ExperimentStore;

pub async fn watch_experiment(
    experiment_store: &ExperimentStore,
    request: PWatchExperimentRequest,
) -> Result<impl Stream<Item=result::Result<PReport, Status>>> {
    let id = request.id.into();

    let reports = experiment_store
        .find_one(id).await?
        .watch().await?;

    let reports = reports.map(|report| {
        Ok((&*report).into())
    });

    Ok(reports)
}