use std::result;

use tokio::stream::{Stream, StreamExt};
use tokio::sync::mpsc;
use tonic::Status;

use lib_interop::protocol::core::PReport;
use lib_interop::protocol::for_client::PWatchExperimentRequest;

use crate::backend::{Result, System};

pub async fn watch_experiment(
    system: &System,
    request: PWatchExperimentRequest,
) -> Result<impl Stream<Item=result::Result<PReport, Status>>> {
    let mut report_rx = system
        .find_experiment(request.id.into()).await?
        .watch().await?;

    let (mut tx, rx) = mpsc::channel(4);

    tokio::spawn(async move {
        while let Some(report) = report_rx.next().await {
            let report = (&*report).into();

            if tx.send(Ok(report)).await.is_err() {
                break;
            }
        }
    });

    Ok(rx)
}