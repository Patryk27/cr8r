use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{Assignment, ExperimentId, RunnerId, Scenario};

use crate::backend::{ExperimentCommand, ExperimentCommandRx, Result, System};

pub struct ExperimentActor {
    system: System,
    id: ExperimentId,
    scenarios: Vec<Scenario>,
    heartbeat: DateTime<Utc>,
    state: ExperimentActorState,
}

#[derive(PartialEq)]
enum ExperimentActorState {
    AwaitingRunner,

    Completed,

    Running {
        runner: RunnerId,
    },
}

impl ExperimentActor {
    pub fn new(system: System, id: ExperimentId, scenarios: Vec<Scenario>) -> Self {
        Self {
            system,
            id,
            scenarios,
            heartbeat: Utc::now(),
            state: ExperimentActorState::AwaitingRunner,
        }
    }

    pub async fn start(mut self, mut rx: ExperimentCommandRx) {
        debug!("Experiment actor started, entering the event loop");
        debug!("-> id: {}", self.id);
        debug!("-> scenarios: {}", self.scenarios.len());

        while let Some(cmd) = rx.next().await {
            debug!("Processing command: {:?}", cmd);

            match cmd {
                ExperimentCommand::Start { runner, tx } => {
                    let _ = tx.send(
                        self.do_start(runner).await,
                    );
                }
            }
        }

        debug!("Experiment actor has been orphaned, halting it");
    }

    async fn do_start(&mut self, runner: RunnerId) -> Result<Assignment> {
        match &self.state {
            ExperimentActorState::AwaitingRunner => {
                self.state = ExperimentActorState::Running { runner };

                Ok(Assignment {
                    experiment_id: self.id.clone(),
                    experiment_scenarios: self.scenarios.clone(),
                })
            }

            ExperimentActorState::Completed => {
                Err("This experiment has been already completed".into())
            }

            ExperimentActorState::Running { runner } => {
                Err(format!(
                    "This experiment is already running on runner `{}`; if the runner's crashed, please wait a few minutes before trying again",
                    runner,
                ).into())
            }
        }
    }
}