use std::time::Duration;

use colored::Colorize;
use log::*;
use tokio::timer;

use lib_sandbox::SandboxProvider;

use crate::backend::ExperimentExecutor;
use crate::core::{Result, SessionClient};

pub struct SystemActor {
    sandbox_provider: SandboxProvider,
    client: SessionClient,
}

impl SystemActor {
    pub fn new(sandbox_provider: SandboxProvider, client: SessionClient) -> Self {
        Self { sandbox_provider, client }
    }

    pub async fn start(mut self) -> Result<()> {
        debug!("Actor started");

        let (assignment, client) = loop {
            debug!("Polling controller for an assignment");

            match self.client.request_assignment().await {
                Ok(Some(assignment)) => {
                    info!(
                        "We've been assigned experiment `{}`",
                        assignment.0.experiment_id.to_string().green(),
                    );

                    break assignment;
                }

                Ok(None) => {
                    timer::delay_for(Duration::from_secs(5)).await;
                }

                Err(err) => {
                    error!("Failed to ask controller for an assignment: {:?}", err);
                    error!("We'll try again in a moment");

                    timer::delay_for(Duration::from_secs(60)).await;
                }
            }
        };

        let sandbox = self.sandbox_provider.provide(
            format!("cr8r-{}", assignment.experiment_id)
        );

        let executor = ExperimentExecutor::spawn(
            sandbox, assignment, client,
        );

        loop {
            executor.status().await;
            timer::delay_for(Duration::from_secs(5)).await;
        }

        // Notify controller that we're starting the experiment
//        client
//            .report_experiment_started()
//            .await?;
//
//        for _ in assignment.experiment_scenarios {
//            client.report_scenario_started().await?;
//
//            for i in 0.. {
//                client.report_output(format!("hello: {}", i)).await?;
//                timer::delay_for(Duration::from_secs(2)).await;
//            }
//
//            client.report_scenario_completed(true).await?;
//        }
//
//        client.report_experiment_completed().await?;

        Ok(())
    }
}