use std::convert::TryInto;

use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_channel::{SendTo, URx};
use lib_interop::domain::{DExperimentId, DReport};
use lib_interop::proto::services::PWatchExperimentRequest;

use crate::modules::app::AppContext;

pub struct ExperimentWatcher<'c> {
    ctxt: &'c mut AppContext,
}

impl<'c> ExperimentWatcher<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> Self {
        Self { ctxt }
    }

    pub async fn watch(&mut self, id: DExperimentId) -> Result<URx<Result<DReport>>> {
        let mut reports = self.ctxt
            .experiments()
            .await?
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

                if let Some(report) = report {
                    report.send_to(&tx);
                }
            }
        });

        Ok(rx)
    }
}