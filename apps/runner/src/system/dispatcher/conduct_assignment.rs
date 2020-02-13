use anyhow::*;
use log::*;
use tokio::time;

use lib_interop::domain::DAssignment;

use crate::system::{Dispatcher, Executor, ExecutorStatus, Logger};

impl Dispatcher {
    pub(super) async fn conduct_assignment(&mut self, assignment: DAssignment) -> Result<()> {
        let sandbox = {
            debug!("Preparing sandbox");

            self.sandbox_provider
                .create(self.sandbox_config.clone())
                .await?
        };

        let logger = {
            debug!("Preparing logger");

            Logger::new(
                self.session.clone(),
                assignment.experiment.id,
            )
        };

        let executor = {
            debug!("Preparing executor");

            Executor::new(
                self.session.clone(),
                assignment,
                sandbox,
                logger,
            )
        };

        debug!("Waiting for experiment to finish");

        loop {
            match executor.get_status().await {
                ExecutorStatus::Aborted => {
                    debug!("Experiment result: aborted");
                    break;
                }

                ExecutorStatus::Completed => {
                    debug!("Experiment result: completed");
                    break;
                }

                ExecutorStatus::Running => {
                    time::delay_for(time::Duration::from_secs(1))
                        .await;
                }
            }
        }

        Ok(())
    }
}