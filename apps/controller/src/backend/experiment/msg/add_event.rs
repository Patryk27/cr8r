use std::sync::Arc;

use chrono::Utc;

use lib_protocol::core::{PExperimentEvent, PExperimentReport, PRunnerId};
use lib_protocol::core::p_experiment_event::Op;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn process(actor: &mut ExperimentActor, runner: PRunnerId, event: PExperimentEvent) -> Result<()> {
    match &mut actor.status {
        ExperimentStatus::AwaitingRunner { .. } => {
            Err("This experiment has not yet been started".into())
        }

        ExperimentStatus::Running {
            runner: experiment_runner,
            events,
            reports,
            completed_scenarios,
            ..
        } => {
            if &runner != experiment_runner {
                return Err("Given runner is not allowed to report on this experiment".into());
            }

            let event = Arc::new(event);

            events.push(Arc::clone(&event));

            if let Some(report) = event_as_report(&event).map(Arc::new) {
                for watcher in &actor.watchers {
                    let _ = watcher.unbounded_send(Arc::clone(&report));
                }

                reports.push(report);
            }

            // There are a few events that convey somewhat special meaning - for instance when we receive "experiment
            // completed", we have to adjust this actor's state accordingly
            if let Some(op) = &event.op {
                match op {
                    Op::ExperimentCompleted(_) => {
                        let success = events
                            .iter()
                            .filter_map(|event| match event.op.as_ref()? {
                                Op::ScenarioCompleted(op) => Some(op),
                                _ => None,
                            })
                            .all(|op| op.success);

                        actor.status = ExperimentStatus::Completed {
                            since: Utc::now(),
                            reports: reports.to_vec(),
                            success,
                        };

                        // Since the experiment's done, no more data will be fed to the watchers and so there's no point
                        // in keeping them alive
                        kill_watchers(actor);
                    }

                    Op::ExperimentAborted(_) => {
                        actor.status = ExperimentStatus::Aborted {
                            since: Utc::now(),
                        };

                        // Since the experiment's done, no more data will be fed to the watchers and so there's no point
                        // in keeping them alive
                        kill_watchers(actor);
                    }

                    Op::ScenarioCompleted(_) => {
                        *completed_scenarios += 1;
                    }

                    _ => (),
                }
            }

            Ok(())
        }

        ExperimentStatus::Completed { .. } => {
            Err("This experiment has been already completed".into())
        }

        ExperimentStatus::Aborted { .. } => {
            Err("This experiment has been aborted".into())
        }

        ExperimentStatus::Zombie { .. } => {
            Err("This experiment has been abandoned by its runner and has become a zombie - it can be manually aborted or restarted".into())
        }
    }
}

fn event_as_report(event: &PExperimentEvent) -> Option<PExperimentReport> {
    use lib_protocol::core::{
        p_experiment_event as p_event,
        p_experiment_report as p_report,
    };

    let (kind, message) = match event.op.as_ref()? {
        p_event::Op::Ping(_) => {
            return None;
        }

        p_event::Op::CustomMessage(p_event::PCustomMessage { message }) => {
            (p_report::Kind::CustomMessage, message.as_str())
        }

        Op::ProcessOutput(p_event::PProcessOutput { line }) => {
            (p_report::Kind::ProcessOutput, line.as_str())
        }

        Op::ExperimentStarted(_) => {
            (p_report::Kind::SystemMessage, "Experiment started")
        }

        Op::ExperimentCompleted(_) => {
            (p_report::Kind::SystemMessage, "Experiment completed")
        }

        Op::ExperimentAborted(_) => {
            (p_report::Kind::SystemMessage, "Experiment aborted")
        }

        Op::ScenarioStarted(_) => {
            (p_report::Kind::SystemMessage, "Scenario started")
        }

        Op::ScenarioCompleted(_) => {
            (p_report::Kind::SystemMessage, "Scenario completed")
        }
    };

    Some(PExperimentReport {
        created_at: event.created_at.clone(),
        kind: kind as _,
        message: message.to_owned(),
    })
}

fn kill_watchers(actor: &mut ExperimentActor) {
    actor.watchers.clear();
    actor.watchers.shrink_to_fit();
}