use std::time::Duration;

use colored::Colorize;
use log::*;
use tokio::timer;

use lib_protocol::core::PAssignment;
use lib_sandbox::{SandboxDef, SandboxProvider};

use crate::backend::{ExperimentExecutor, ExperimentExecutorStatus};
use crate::core::{ExperimentClient, Result, SandboxConfig, SessionClient};

pub struct SystemActor {
    sandbox_config: SandboxConfig,
    sandbox_provider: SandboxProvider,
    client: SessionClient,
}

impl SystemActor {
    pub fn new(
        sandbox_config: SandboxConfig,
        sandbox_provider: SandboxProvider,
        client: SessionClient,
    ) -> Self {
        Self {
            sandbox_config,
            sandbox_provider,
            client,
        }
    }

    pub async fn main(mut self) -> Result<()> {
        debug!("Actor started");

        loop {
            let (assignment, client) = self
                .poll_for_assignment()
                .await;

            match self.conduct_experiment(assignment, client).await {
                Ok(_) => {
                    debug!("Experiment conducted successfully");
                }

                Err(err) => {
                    // @todo we should notify controller about this incident

                    error!("Couldn't conduct experiment: {:?}", err);
                }
            }
        }
    }

    async fn poll_for_assignment(&mut self) -> (PAssignment, ExperimentClient) {
        loop {
            debug!("Polling controller for an assignment");

            match self.client.request_assignment().await {
                Ok(Some(assignment)) => {
                    info!(
                        "We've been assigned experiment `{}`",
                        assignment.0.experiment_id.to_string().green(),
                    );

                    return assignment;
                }

                Ok(None) => {
                    debug!("Got nothing");
                    debug!("We'll try again in a moment");

                    timer::delay_for(Duration::from_secs(5))
                        .await;
                }

                Err(err) => {
                    error!("Couldn't ask controller for an assignment: {:?}", err);
                    error!("We'll try again in a moment");

                    timer::delay_for(Duration::from_secs(60))
                        .await;
                }
            }
        }
    }

    async fn conduct_experiment(&mut self, assignment: PAssignment, client: ExperimentClient) -> Result<()> {
        debug!("Conducting experiment");

        let sandbox_def = match &self.sandbox_config {
            SandboxConfig::Lxd { container_name } => {
                SandboxDef::Lxd {
                    container: container_name.parse().unwrap(), // @todo
                    image: "ubuntu:18.04".parse().unwrap(), // @todo
                }
            }

            SandboxConfig::Shell { dir } => {
                SandboxDef::Shell {
                    dir: dir.into()
                }
            }
        };

        let sandbox = self.sandbox_provider
            .create(sandbox_def)
            .await?;

        let executor = ExperimentExecutor::spawn(
            sandbox, assignment, client,
        );

        loop {
            match executor.status().await {
                ExperimentExecutorStatus::Aborted | ExperimentExecutorStatus::Completed => {
                    break;
                }

                ExperimentExecutorStatus::Running => {
                    timer::delay_for(Duration::from_secs(1))
                        .await;
                }
            }
        }

        Ok(())
    }
}