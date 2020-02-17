use std::result;

use anyhow::*;
use tokio::stream::{Stream, StreamExt};
use tokio::sync::mpsc::channel;
use tokio::task::spawn;
use tonic::Status;

use lib_interop::proto::models::PReport;
use lib_interop::proto::services::PWatchExperimentRequest;

use crate::system::ExperimentStore;

pub async fn watch_experiment(
    experiment_store: &ExperimentStore,
    request: PWatchExperimentRequest,
) -> Result<impl Stream<Item=result::Result<PReport, Status>>> {
    let id = request.id.into();

    let experiment = experiment_store
        .find_one(id)
        .await?;

    let mut reports = experiment
        .watch()
        .await?;

    let (mut tx, rx) = channel(4);

    spawn(async move {
        while let Some(report) = reports.next().await {
            let report = (&*report).into();

            if tx.send(Ok(report)).await.is_err() {
                break;
            }
        }
    });

    Ok(rx)
}