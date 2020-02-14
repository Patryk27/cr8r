use std::result;

use anyhow::*;
use tokio::stream::{Stream, StreamExt};
use tokio::sync::mpsc::channel;
use tokio::task::spawn;
use tonic::Status;

use lib_interop::proto::controller::PWatchExperimentRequest;
use lib_interop::proto::core::PReport;

use crate::system::System;

pub async fn watch_experiment(
    system: &System,
    request: PWatchExperimentRequest,
) -> Result<impl Stream<Item=result::Result<PReport, Status>>> {
    let id = request.id.into();

    let experiment = system
        .experiments
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