use std::sync::Arc;

use chrono::Utc;

use lib_protocol::core::{PExperimentEvent, PExperimentReport, PRunnerId};
use lib_protocol::core::p_experiment_event::*;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn add_event(actor: &mut ExperimentActor, runner: PRunnerId, event: PExperimentEvent) -> Result<()> {
    match &mut actor.status {
        ExperimentStatus::Idle { .. } => {
            Err("This experiment has not yet been started".into())
        }

        ExperimentStatus::Running {
            runner: experiment_runner,
            events,
            reports,
            completed_steps,
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
                    Op::ExperimentSucceeded(_) => {
                        actor.status = ExperimentStatus::Completed {
                            since: Utc::now(),
                            reports: reports.to_vec(),
                            result: Ok(()),
                        };

                        // Since the experiment's done, no more data will be fed to the watchers and so there's no point
                        // in keeping them alive
                        kill_watchers(actor);
                    }

                    Op::ExperimentFailed(PExperimentFailed { cause }) => {
                        actor.status = ExperimentStatus::Completed {
                            since: Utc::now(),
                            reports: reports.to_vec(),
                            result: Err(cause.to_string()),
                        };

                        // Since the experiment's done, no more data will be fed to the watchers and so there's no point
                        // in keeping them alive
                        kill_watchers(actor);
                    }

                    Op::StepSucceeded(_) | Op::StepFailed(_) => {
                        *completed_steps += 1;
                    }

                    _ => (),
                }
            }

            Ok(())
        }

        ExperimentStatus::Completed { .. } => {
            Err("This experiment has been already completed".into())
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
        Op::Ping(_) => {
            return None;
        }

        Op::SystemMsg(p_event::PSystemMsg { msg }) => {
            (p_report::Kind::SystemMsg, msg.as_str())
        }

        Op::UserMsg(p_event::PUserMsg { msg }) => {
            (p_report::Kind::UserMsg, msg.as_str())
        }

        Op::ProcessOutput(p_event::PProcessOutput { line }) => {
            (p_report::Kind::ProcessOutput, line.as_str())
        }

        Op::ExperimentStarted(_) => {
            (p_report::Kind::SystemMsg, "Experiment started")
        }

        Op::ExperimentSucceeded(_) => {
            (p_report::Kind::SystemMsg, "Experiment completed (result: success)")
        }

        Op::ExperimentFailed(_) => {
            (p_report::Kind::SystemMsg, "Experiment completed (result: failure)")
        }

        Op::StepSucceeded(_) | Op::StepFailed(_) => {
            return None;
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