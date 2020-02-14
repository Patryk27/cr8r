use anyhow::*;
use log::*;
use tokio::time::{delay_for, Duration};

use lib_interop::domain::DExperimentId;

use crate::system::{Dispatcher, Executor, ExecutorStatus, Logger};

impl Dispatcher {
    pub(super) async fn conduct_assignment(&mut self, experiment_id: DExperimentId) -> Result<()> {
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
                experiment_id,
            )
        };

        let executor = {
            debug!("Preparing executor");

            Executor::new(
                self.session.clone(),
                sandbox,
                logger,
                experiment_id,
            )
        };

        debug!("Waiting for experiment to finish");

        loop {
            match executor.get_status().await {
                ExecutorStatus::Running => {
                    delay_for(Duration::from_secs(1))
                        .await;
                }

                ExecutorStatus::Completed => {
                    debug!("Experiment result: completed");
                    break;
                }

                ExecutorStatus::Stopped => {
                    debug!("Experiment result: stopped");
                    break;
                }
            }
        }

        Ok(())
    }
}