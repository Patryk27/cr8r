use std::sync::Arc;

use chrono::Utc;

use lib_interop::contract::{CEvent, CEventType, CReport, CRunnerId};

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn add_event(actor: &mut ExperimentActor, runner: CRunnerId, event: CEvent) -> Result<()> {
    match &mut actor.status {
        ExperimentStatus::Idle { .. } => {
            Err("This experiment has not yet been started".into())
        }

        ExperimentStatus::Running {
            runner: experiment_runner,
            events,
            reports,
            completed_ops,
            ..
        } => {
            if &runner != experiment_runner {
                return Err("Given runner is not allowed to report on this experiment".into());
            }

            let event = Arc::new(event);

            events.push(Arc::clone(&event));

            if let Some(report) = event_as_report(&event).map(Arc::new) {
                for watcher in &actor.watchers {
                    let _ = watcher.send(Arc::clone(&report));
                }

                reports.push(report);
            }

            match &event.ty {
                CEventType::ExperimentSucceeded => {
                    actor.status = ExperimentStatus::Completed {
                        since: Utc::now(),
                        reports: reports.to_vec(),
                        result: Ok(()),
                    };

                    kill_watchers(actor);
                }

                CEventType::ExperimentFailed { cause } => {
                    actor.status = ExperimentStatus::Completed {
                        since: Utc::now(),
                        reports: reports.to_vec(),
                        result: Err(cause.to_string()),
                    };

                    kill_watchers(actor);
                }

                CEventType::OpcodeSucceeded { .. } | CEventType::OpcodeFailed { .. } => {
                    *completed_ops += 1;
                }

                _ => (),
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

fn event_as_report(event: &CEvent) -> Option<CReport> {
    Some(match &event.ty {
        CEventType::SystemMsg { msg } => {
            CReport::system_msg(event.at, msg)
        }

        CEventType::UserMsg { msg } => {
            CReport::user_msg(event.at, msg)
        }

        CEventType::ProcessOutput { line } => {
            CReport::process_output(event.at, line)
        }

        CEventType::ExperimentStarted => {
            CReport::system_msg(event.at, "Experiment started")
        }

        CEventType::ExperimentSucceeded => {
            CReport::system_msg(event.at, "Experiment completed successfully")
        }

        CEventType::ExperimentFailed { cause } => {
            CReport::system_msg(event.at, format!("Experiment completed with failure: {}", cause))
        }

        _ => {
            return None;
        }
    })
}

fn kill_watchers(actor: &mut ExperimentActor) {
    actor.watchers.clear();
    actor.watchers.shrink_to_fit();
}