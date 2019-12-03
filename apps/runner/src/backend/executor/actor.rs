use std::time::Duration;

use futures_util::StreamExt;
use log::*;
use tokio::timer;

use lib_protocol::core::{Assignment, Scenario, ScenarioStep};

use crate::backend::{ExecutorMsg, ExecutorRx, ExecutorStatus};
use crate::core::ExperimentClient;

pub struct ExecutorActor {
    rx: ExecutorRx,
    assignment: Assignment,
    client: ExperimentClient,
    status: ExecutorStatus,
}

// @todo this actor must not fail
impl ExecutorActor {
    pub fn new(rx: ExecutorRx, assignment: Assignment, client: ExperimentClient) -> Self {
        Self {
            rx,
            assignment,
            client,
            status: ExecutorStatus::Running,
        }
    }

    pub async fn start(mut self) {
        debug!("Actor started");
        debug!("-> experiment id: {}", self.assignment.experiment_id);
        debug!("-> experiment scenarios: {}", self.assignment.experiment_scenarios.len());

        self.client
            .report_experiment_started()
            .await
            .unwrap();

        let scenarios = self.assignment
            .experiment_scenarios
            .drain(..)
            .collect(): Vec<_>;

        for scenario in scenarios {
            self.execute_scenario(scenario).await;
        }

        self.client
            .report_experiment_completed()
            .await
            .unwrap();

        debug!("Actor orphaned, halting");
    }

    async fn execute_scenario(&mut self, mut scenario: Scenario) {
        let steps = scenario.steps
            .drain(..)
            .collect(): Vec<_>;

        self.client
            .report_scenario_started()
            .await
            .unwrap();

        for step in steps {
            self.execute_step(step).await;
        }

        self.client
            .report_scenario_completed(true) // @todo un-hardcode the `true`
            .await
            .unwrap();
    }

    async fn execute_step(&mut self, step: ScenarioStep) {
        self.process_messages().await;

        for i in 1.. {
            self.client.report_output(format!("i={}", i))
                .await
                .unwrap();

            timer::delay_for(Duration::from_millis(250))
                .await;
        }
    }

    async fn process_messages(&mut self) {
        while let Ok(Some(msg)) = self.rx.try_next() {
            match msg {
                ExecutorMsg::Status { tx } => {
                    let _ = tx.send(self.status);
                }
            }
        }
    }

    fn status(&self) -> ExecutorStatus {
        self.status
    }
}