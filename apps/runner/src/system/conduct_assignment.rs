use anyhow::Result;
use log::*;
use tokio::time;

use lib_interop::domain::DAssignment;
use lib_sandbox::SandboxDef;

use crate::core::SandboxConfig;
use crate::experiment::{ExperimentExecutor, ExperimentExecutorStatus, ExperimentLogger};
use crate::system::System;

impl System {
    pub(super) async fn conduct_assignment(&mut self, assignment: DAssignment) -> Result<()> {
        let sandbox = {
            debug!("Preparing sandbox");

            let sandbox_def = match &self.sandbox_config {
                SandboxConfig::Lxd { container_name } => {
                    SandboxDef::Lxd {
                        container: container_name.parse().unwrap(), // @todo
                        image: "ubuntu:18.04".parse().unwrap(), // @todo
                    }
                }

                SandboxConfig::Shell { root } => {
                    SandboxDef::Shell {
                        root: root.into(),
                    }
                }
            };

            self.sandbox_provider
                .create(sandbox_def)
                .await?
        };

        let logger = {
            debug!("Preparing experiment logger");

            ExperimentLogger::new(
                self.session.clone(),
                assignment.experiment.id.clone(),
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