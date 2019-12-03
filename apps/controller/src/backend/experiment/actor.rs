use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{self, Assignment, ExperimentId, Report, RunnerId, Scenario};

use crate::backend::{ExperimentMsg, ExperimentRx, ExperimentWatcher, Result, System};

pub struct ExperimentActor {
    rx: ExperimentRx,
    system: System,
    experiment: ExperimentId,
    scenarios: Vec<Scenario>,
    created_at: DateTime<Utc>,
    heartbeaten_at: DateTime<Utc>,
    watcher: Option<ExperimentWatcher>,
    status: ExperimentActorStatus,
}

#[derive(PartialEq)]
enum ExperimentActorStatus {
    AwaitingRunner {
        since: DateTime<Utc>,
    },

    Completed {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        runner: RunnerId,
        reports: Vec<Report>,
        completed_scenarios: u32,
        total_scenarios: u32,
    },

    Zombie {
        since: DateTime<Utc>,
    },
}

impl ExperimentActor {
    pub fn new(
        rx: ExperimentRx,
        system: System,
        experiment: ExperimentId,
        scenarios: Vec<Scenario>,
    ) -> Self {
        Self {
            rx,
            system,
            experiment,
            scenarios,
            created_at: Utc::now(),
            heartbeaten_at: Utc::now(),
            watcher: None,

            status: ExperimentActorStatus::AwaitingRunner {
                since: Utc::now(),
            },
        }
    }

    pub async fn start(mut self) {
        debug!("Actor started, entering event loop");
        debug!("-> experiment: {}", self.experiment);
        debug!("-> scenarios: {}", self.scenarios.len());

        while let Some(msg) = self.rx.next().await {
            debug!("Processing message: {:?}", msg);

            match msg {
                ExperimentMsg::AsModel { tx } => {
                    let _ = tx.send(self.as_model());
                }

                ExperimentMsg::Report { runner, report, tx } => {
                    let _ = tx.send(self.report(runner, report));
                }

                ExperimentMsg::Start { runner, tx } => {
                    let _ = tx.send(self.start_(runner));
                }

                ExperimentMsg::Watch { tx } => {
                    let _ = tx.send(self.watch());
                }
            }
        }

        debug!("Actor orphaned, halting");
    }

    fn as_model(&self) -> core::Experiment {
        use self::core::experiment::status;

        let status = match &self.status {
            ExperimentActorStatus::AwaitingRunner { since } => {
                status::Op::AwaitingRunner(status::AwaitingRunner {
                    since: since.to_rfc3339(),
                })
            }

            ExperimentActorStatus::Completed { since } => {
                status::Op::Completed(status::Completed {
                    since: since.to_rfc3339(),
                })
            }

            ExperimentActorStatus::Running { since, runner, completed_scenarios, total_scenarios, .. } => {
                status::Op::Running(status::Running {
                    since: since.to_rfc3339(),
                    runner_id: runner.to_owned(),
                    completed_scenarios: *completed_scenarios,
                    total_scenarios: *total_scenarios,
                })
            }

            ExperimentActorStatus::Zombie { since } => {
                status::Op::Zombie(status::Zombie {
                    since: since.to_rfc3339(),
                })
            }
        };

        core::Experiment {
            id: self.experiment.clone(),
            created_at: self.created_at.to_rfc3339(),
            heartbeaten_at: self.heartbeaten_at.to_rfc3339(),

            status: Some(core::experiment::Status {
                op: Some(status),
            }),
        }
    }

    fn report(&mut self, runner: RunnerId, report: Report) -> Result<()> {
        match &mut self.status {
            ExperimentActorStatus::AwaitingRunner { .. } => {
                Err("This experiment has not yet been started".into())
            }

            ExperimentActorStatus::Completed { .. } => {
                Err("This experiment has been already completed".into())
            }

            ExperimentActorStatus::Running {
                runner: st_runner,
                reports: st_reports,
                total_scenarios: st_total_scenarios,
                completed_scenarios: st_completed_scenarios,
                ..
            } => {
                if &runner != st_runner {
                    return Err("Specified runner is not allowed to report on this experiment".into());
                }

                if let Some(watcher) = &mut self.watcher {
                    watcher.add(report.clone());
                }

                st_reports.push(report);

                // @todo increase total / completed

                Ok(())
            }

            ExperimentActorStatus::Zombie { .. } => {
                Err("This experiment has been abandoned by its runner and has become a zombie - it can be only aborted or restarted".into())
            }
        }
    }

    fn start_(&mut self, runner: RunnerId) -> Result<Assignment> {
        match &self.status {
            ExperimentActorStatus::AwaitingRunner { .. } => {
                self.status = ExperimentActorStatus::Running {
                    since: Utc::now(),
                    runner,
                    reports: Vec::new(),
                    total_scenarios: 0,
                    completed_scenarios: 0,
                };

                Ok(Assignment {
                    experiment_id: self.experiment.clone(),
                    experiment_scenarios: self.scenarios.clone(),
                })
            }

            ExperimentActorStatus::Completed { .. } => {
                Err("This experiment has been already completed".into())
            }

            ExperimentActorStatus::Running { runner, .. } => {
                Err(format!(
                    "This experiment is already running on runner `{}`; if the runner's crashed, please wait a few minutes before trying again",
                    runner,
                ).into())
            }

            ExperimentActorStatus::Zombie { .. } => {
                unimplemented!()
            }
        }
    }

    fn watch(&mut self) -> ExperimentWatcher {
        if let Some(mut watcher) = self.watcher.take() {
            watcher.kill();
        }

        let watcher = ExperimentWatcher::spawn();

        // @todo allow handling many watchers at once
        self.watcher = Some(watcher.clone());

        watcher
    }
}