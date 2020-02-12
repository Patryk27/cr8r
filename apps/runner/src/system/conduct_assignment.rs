use anyhow::*;
use log::*;
use tokio::time;

use lib_interop::domain::DAssignment;

use crate::experiment::{ExperimentExecutor, ExperimentExecutorStatus, ExperimentLogger};
use crate::system::System;

impl System {
    pub(super) async fn conduct_assignment(&mut self, assignment: DAssignment) -> Result<()> {
        let sandbox = {
            debug!("Preparing sandbox");

            self.sandbox_provider
                .create(self.sandbox_config.clone())
                .await?
        };

        let logger = {
            debug!("Preparing experiment logger");

            ExperimentLogger::new(
                self.session.clone(),
                assignment.experiment.id,
            )
        };

        let executor = {
            debug!("Preparing experiment executor");

            ExperimentExecutor::new(
                self.session.clone(),
                assignment,
                sandbox,
                logger,
            )
        };

        debug!("Waiting for experiment to finish");

        loop {
            // @todo self.process_messages();

            match executor.get_status().await {
                ExperimentExecutorStatus::Aborted | ExperimentExecutorStatus::Completed => {
                    break;
                }

                ExperimentExecutorStatus::Running => {
                    time::delay_for(time::Duration::from_secs(1))
                        .await;
                }
            }
        }

        Ok(())
    }
}