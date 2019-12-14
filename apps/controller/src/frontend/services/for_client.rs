use tokio::sync::mpsc;
use tonic::{Code, Request, Response, Status};

use lib_protocol::core::PExperimentDefinition;
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
        _: Request<PFindExperimentsRequest>,
    ) -> Result<Response<PFindExperimentsReply>, Status> {
        let mut experiments = Vec::new();

        for experiment in self.system.find_experiments().await {
            let experiment = experiment
                .as_model()
                .await;

            experiments.push(experiment);
        }

        Ok(Response::new(PFindExperimentsReply { experiments }))
    }

    async fn launch_experiment(
        &self,
        request: Request<PLaunchExperimentRequest>,
    ) -> Result<Response<PLaunchExperimentReply>, Status> {
        let request = request.into_inner();

        if let Some(PExperimentDefinition { op: Some(experiment) }) = request.experiment {
            let id = self.system
                .launch_experiment(experiment)
                .await?;

            Ok(Response::new(PLaunchExperimentReply { id }))
        } else {
            Err(Status::new(Code::Internal, "No experiment has been provided"))
        }
    }

    type WatchExperimentStream = mpsc::Receiver<Result<PWatchExperimentReply, Status>>;

    async fn watch_experiment(
        &self,
        request: Request<PWatchExperimentRequest>,
    ) -> Result<Response<Self::WatchExperimentStream>, Status> {
        let request = request.into_inner();

        let mut watcher = self.system
            .find_experiment(request.id).await?
            .watch().await?;

        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            loop {
                let reply = watcher
                    .pull_reply()
                    .await;

                if tx.send(Ok(reply)).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(rx))
    }

    async fn find_runners(
        &self,
        _: Request<PFindRunnersRequest>,
    ) -> Result<Response<PFindRunnersReply>, Status> {
        let mut runners = Vec::new();

        for runner in self.system.find_runners().await {
            let runner = runner
                .as_model()
                .await;

            runners.push(runner);
        }

        Ok(Response::new(PFindRunnersReply { runners }))
    }
}