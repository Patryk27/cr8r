use std::sync::Arc;

use chrono::Utc;
use log::*;

use lib_interop::contract::{CEvent, CEventType, CJob, CReport, CRunnerId};

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
            completed_jobs,
            ..
        } => {
            if &runner != experiment_runner {
                return Err("Given runner is not allowed to report on this experiment".into());
            }

            let event = Arc::new(event);

            events.push(Arc::clone(&event));

            if let Some(report) = event_as_report(&actor.jobs, &event).map(Arc::new) {
                for watcher in &actor.watchers {
                    let _ = watcher.send(Arc::clone(&report));
                }

                reports.push(report);
            }

            match &event.ty {
                CEventType::ExperimentCompleted => {
                    let result = {
                        let mut result = Ok(());

                        for event in events {
                            if let CEventType::JobCompleted { id, result: Err(err) } = &event.ty {
                                result = Err(format!("job #{} failed: {}", id, err));
                                break;
                            }
                        }

                        result
                    };

                    actor.status = ExperimentStatus::Completed {
                        since: Utc::now(),
                        reports: reports.to_vec(),
                        result,
                    };

                    kill_watchers(actor);
                }

                CEventType::JobCompleted { .. } => {
                    *completed_jobs += 1;
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

fn event_as_report(jobs: &[CJob], event: &CEvent) -> Option<CReport> {
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

        CEventType::ExperimentCompleted => {
            CReport::system_msg(event.at, "Experiment completed")
        }

        CEventType::JobStarted { id } => {
            if let Some(job) = jobs.get(*id) {
                CReport::system_msg(event.at, format!("Job `{}` started", job.name))
            } else {
                warn!("Runner reported that it has started working on job #{}, which does not exist; this is probably a bug", id);
                return None;
            }
        }

        CEventType::JobCompleted { id, result } => {
            let result = if let Err(err) = result {
                format!("failure: {}", err)
            } else {
                "success".to_string()
            };

            if let Some(job) = jobs.get(*id) {
                CReport::system_msg(event.at, format!("Job `{}` completed; result: {}", job.name, result))
            } else {
                warn!("Runner reported that it has finished working on job #{}, which does not exist; this is probably a bug", id);
                return None;
            }
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