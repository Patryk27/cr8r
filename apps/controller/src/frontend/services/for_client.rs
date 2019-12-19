use futures_util::StreamExt;
use tokio::sync::mpsc;
use tonic::{Code, Request, Response, Status};

use lib_protocol::core::{PExperimentDef, PExperimentReport};
use lib_protocol::for_client::*;
use lib_protocol::for_client::server::ForClient;

use crate::backend::System;

pub struct ForClientService {
    system: System,
}

impl ForClientService {
    pub fn new(system: System) -> Self {
        Self { system }
    }
}

// @todo validate client's secret key
#[tonic::async_trait]
impl ForClient for ForClientService {
    async fn hello(
        &self,
        _: Request<PHelloRequest>,
    ) -> Result<Response<PHelloReply>, Status> {
        Ok(Response::new(PHelloReply {
            version: "0.1.0".into(),
            uptime: 0, // @todo
        }))
    }

    async fn find_experiments(
        &self,
        request: Request<PFindExperimentsRequest>,
    ) -> Result<Response<PFindExperimentsReply>, Status> {
        // @todo filtering should happen inside `system`, not here
        let request = request.into_inner();

        let mut experiments = Vec::new();

        for experiment in self.system.find_experiments().await {
            let experiment = experiment
                .get_model()
                .await;

            let mut matches = true;

            if !request.filter_id.is_empty() {
                matches = experiment.id == request.filter_id;
            }

            if matches {
                experiments.push(experiment);
            }
        }

        Ok(Response::new(PFindExperimentsReply { experiments }))
    }

    async fn launch_experiment(
        &self,
        request: Request<PLaunchExperimentRequest>,
    ) -> Result<Response<PLaunchExperimentReply>, Status> {
        let request = request.into_inner();

        if let Some(PExperimentDef { op: Some(experiment_def) }) = request.experiment_def {
            let id = self.system
                .launch_experiment(experiment_def)
                .await?;

            Ok(Response::new(PLaunchExperimentReply { id }))
        } else {
            Err(Status::new(Code::Internal, "No experiment has been provided"))
        }
    }

    type WatchExperimentStream = mpsc::Receiver<Result<PExperimentReport, Status>>;

    async fn watch_experiment(
        &self,
        request: Request<PWatchExperimentRequest>,
    ) -> Result<Response<Self::WatchExperimentStream>, Status> {
        let request = request.into_inner();

        let mut report_rx = self.system
            .find_experiment(request.id).await?
            .watch().await?;

        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            while let Some(report) = report_rx.next().await {
                let report = (&*report).to_owned();

                if tx.send(Ok(report)).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(rx))
    }

    async fn find_experiment_reports(
        &self,
        request: Request<PFindExperimentReportsRequest>,
    ) -> Result<Response<PFindExperimentReportsReply>, Status> {
        let request = request.into_inner();

        let reports = self.system
            .find_experiment(request.filter_experiment_id).await?
            .get_reports().await
            .into_iter()
            .map(|report| (&*report).to_owned())
            .collect();

        Ok(Response::new(PFindExperimentReportsReply { reports }))
    }

    async fn find_runners(
        &self,
        _: Request<PFindRunnersRequest>,
    ) -> Result<Response<PFindRunnersReply>, Status> {
        let mut runners = Vec::new();

        for runner in self.system.find_runners().await {
            let runner = runner
                .get_model()
                .await;

            runners.push(runner);
        }

        Ok(Response::new(PFindRunnersReply { runners }))
    }
}