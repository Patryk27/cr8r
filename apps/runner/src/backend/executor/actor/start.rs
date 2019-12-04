use log::*;

use crate::backend::ExecutorActor;

impl ExecutorActor {
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
            self.client
                .report_scenario_started()
                .await
                .unwrap();

            let scenario_succeeded = match self.execute_scenario(scenario).await {
                Ok(()) => {
                    true
                }

                Err(err) => {
                    self.client
                        .report_message(format!("Scenario failed: {}", err))
                        .await
                        .unwrap();

                    false
                }
            };

            self.client
                .report_scenario_completed(scenario_succeeded)
                .await
                .unwrap();
        }

        self.client
            .report_experiment_completed()
            .await
            .unwrap();

        debug!("Actor orphaned, halting");
    }
}