use tokio::sync::mpsc;
use tonic::{Code, Request, Response, Status};

use lib_protocol::client::*;
use lib_protocol::client::server::Client;
use lib_protocol::core::ExperimentDefinition;

use crate::backend::System;

pub struct ClientService {
    system: System,
}

impl ClientService {
    pub fn new(system: System) -> Self {
        Self { system }
    }
}

// @todo validate client's secret key
#[tonic::async_trait]
impl Client for ClientService {
    async fn hello(&self, _: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            version: "0.1.0".into(),
            uptime: 0, // @todo
        }))
    }

    async fn find_experiments(
        &self,
        _: Request<FindExperimentsRequest>,
    ) -> Result<Response<FindExperimentsReply>, Status> {
        let mut experiments = Vec::new();

        for experiment in self.system.find_experiments().await {
            experiments.push(
                experiment.as_model().await,
            );
        }

        Ok(Response::new(FindExperimentsReply {
            experiments,
        }))
    }

    async fn launch_experiment(
        &self,
        request: Request<LaunchExperimentRequest>,
    ) -> Result<Response<LaunchExperimentReply>, Status> {
        let request = request.into_inner();

        if let Some(ExperimentDefinition { experiment_definition_inner: Some(experiment) }) = request.experiment {
            let id = self.system.launch_experiment(experiment).await?;

            Ok(Response::new(LaunchExperimentReply {
                id,
                position_in_queue: 1, // @todo
            }))
        } else {
            Err(Status::new(Code::Internal, "No experiment has been provided"))
        }
    }

    type WatchExperimentStream = mpsc::Receiver<Result<WatchExperimentReply, Status>>;

    async fn watch_experiment(
        &self,
        request: Request<WatchExperimentRequest>,
    ) -> Result<Response<Self::WatchExperimentStream>, Status> {
        let request = request.into_inner();

        let mut watcher = self.system
            .find_experiment(request.experiment_id).await?
            .watch().await;

        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            while let Some(line) = watcher.get().await {
                let reply = Ok(WatchExperimentReply {
                    line,
                });

                if tx.send(reply).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(rx))
    }

    async fn find_runners(&self, _: Request<FindRunnersRequest>) -> Result<Response<FindRunnersReply>, Status> {
        let mut runners = Vec::new();

        for runner in self.system.find_runners().await {
            runners.push(
                runner.as_model().await,
            );
        }

        Ok(Response::new(FindRunnersReply {
            runners,
        }))
    }
}