use std::convert::TryInto;

use anyhow::*;
use tokio::sync::mpsc;
use tokio::task;

use lib_core_channel::{SendTo, URx};
use lib_interop::domain::{DExperimentId, DReport};

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
            .client()
            .await?
            .watch_experiment(id.into())
            .await?;

        let (tx, rx) = mpsc::unbounded_channel();

        task::spawn(async move {
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