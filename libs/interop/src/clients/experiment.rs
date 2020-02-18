use std::convert::TryInto;

use anyhow::*;
use tokio::stream::Stream;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;
use tonic::transport::Channel;

use lib_core_channel::SendTo;

use crate::connection::Connection;
use crate::conv;
use crate::models::{DDefinition, DExperiment, DExperimentId, DReport};
use crate::proto::services::*;
use crate::proto::services::experiments_client::ExperimentsClient as ExperimentsClientInner;

#[derive(Clone)]
pub struct ExperimentClient {
    inner: ExperimentsClientInner<Channel>,
}

impl ExperimentClient {
    crate fn new(conn: Connection) -> Self {
        Self {
            inner: ExperimentsClientInner::with_interceptor(
                conn.channel,
                conn.interceptor,
            ),
        }
    }

    pub async fn create(&mut self, definition: DDefinition) -> Result<DExperimentId> {
        let reply = self.inner
            .create_experiment(PCreateExperimentRequest {
                definition: Some(definition.into()),
            })
            .await?
            .into_inner();

        Ok(reply.id.into())
    }

    pub async fn delete(&mut self, id: DExperimentId) -> Result<()> {
        self.inner
            .delete_experiment(PDeleteExperimentRequest { id: id.into() })
            .await?;

        Ok(())
    }

    pub async fn find_one(&mut self, id: DExperimentId) -> Result<DExperiment> {
        let mut experiments = self.inner
            .find_experiments(PFindExperimentsRequest { id: id.into() })
            .await?
            .into_inner()
            .experiments;

        let mut experiments = experiments.drain(..);

        if let Some(experiment) = experiments.next() {
            Ok(experiment.try_into()?)
        } else {
            bail!("No such experiment exists");
        }
    }

    pub async fn find_many(&mut self) -> Result<Vec<DExperiment>> {
        let experiments = self.inner
            .find_experiments(PFindExperimentsRequest::default())
            .await?
            .into_inner()
            .experiments;

        Ok(conv!(experiments as [_?]))
    }

    pub async fn stop(&mut self, id: DExperimentId) -> Result<()> {
        self.inner
            .stop_experiment(PStopExperimentRequest { id: id.into() })
            .await?;

        Ok(())
    }

    pub async fn watch(&mut self, id: DExperimentId) -> Result<impl Stream<Item=Result<DReport>>> {
        let mut reports = self.inner
            .watch_experiment(PWatchExperimentRequest { id: id.into() })
            .await?
            .into_inner();

        let (tx, rx) = unbounded_channel();

        spawn(async move {
            loop {
                let report = reports
                    .message()
                    .await;

                let report = report
                    .transpose()
                    .map(|report| match report {
                        Ok(report) => Ok(report.try_into()?),
                        Err(err) => Err(err.into()),
                    });

                match report {
                    Some(report) => {
                        report.send_to(&tx);
                    }

                    None => {
                        break;
                    }
                }
            }
        });

        Ok(rx)
    }
}