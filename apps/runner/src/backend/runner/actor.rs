use std::time::Duration;

use colored::Colorize;
use log::*;
use tokio::timer;

use crate::core::{Result, SessionClient};

pub struct RunnerActor {
    session_client: SessionClient,
}

impl RunnerActor {
    pub fn new(session_client: SessionClient) -> Self {
        Self { session_client }
    }

    pub async fn start(mut self) -> Result<()> {
        let (assignment, mut experiment_client) = loop {
            debug!("Polling controller for an assignment");

            if let Some((assignment, experiment_client)) = self.session_client.request_assignment().await? {
                info!(
                    "We've been assigned experiment `{}`",
                    assignment.experiment_id.to_string().green(),
                );

                break (assignment, experiment_client);
            }

            timer::delay_for(Duration::from_secs(1)).await;
        };

        experiment_client.report_experiment_started().await?;

        for _ in assignment.experiment_scenarios {
            experiment_client.report_scenario_started().await?;

            for i in 0.. {
                experiment_client.report_output(format!("hello: {}", i)).await?;
                timer::delay_for(Duration::from_secs(2)).await;
            }

            experiment_client.report_scenario_completed(true).await?;
        }

        experiment_client.report_experiment_completed().await?;

        Ok(())
    }
}