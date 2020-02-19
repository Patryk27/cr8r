use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::*;
use chrono::Utc;
use log::*;

use lib_interop::models::{DEvent, DEventType, DJob, DReport, DRunnerId};
use lib_interop::models::job::{DJobId, DJobStatus};

use super::super::{ExperimentActor, ExperimentStatus};

pub fn add_event(actor: &mut ExperimentActor, actual_runner_id: DRunnerId, event: DEvent) -> Result<()> {
    match &mut actor.status {
        ExperimentStatus::Idle { .. } => {
            Err(anyhow!("This experiment has not yet been started"))
        }

        ExperimentStatus::Running {
            runner_id,
            events,
            reports,
            completed_jobs,
            ..
        } => {
            if *runner_id != actual_runner_id {
                return Err(anyhow!("Given runner is not allowed to report on this experiment"));
            }

            let event = Arc::new(event);

            events.push(Arc::clone(&event));

            if let Some(report) = event_to_report(&actor.jobs, &event).map(Arc::new) {
                for watcher in &actor.watchers {
                    let _ = watcher.send(Arc::clone(&report));
                }

                reports.push(report);
            }

            match &event.ty {
                DEventType::ExperimentCompleted => {
                    let result = {
                        let mut result = Ok(());

                        for event in events {
                            if let DEventType::JobCompleted { id, result: Err(err) } = &event.ty {
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

                DEventType::JobCompleted { id, result } => {
                    *completed_jobs += 1;

                    if let Some(job) = actor.jobs.get_mut(&id) {
                        job.status = DJobStatus::Completed {
                            result: result.to_owned(),
                        };
                    }
                }

                _ => (),
            }

            Ok(())
        }

        ExperimentStatus::Completed { .. } => {
            Err(anyhow!("This experiment has been already completed"))
        }

        ExperimentStatus::Stopped { .. } => {
            Err(anyhow!("This experiment has been already stopped"))
        }
    }
}

fn event_to_report(jobs: &BTreeMap<DJobId, DJob>, event: &DEvent) -> Option<DReport> {
    Some(match &event.ty {
        DEventType::SystemMsg { msg } => {
            DReport::system_msg(event.at, msg)
        }

        DEventType::ProcessMsg { msg } => {
            DReport::process_msg(event.at, msg)
        }

        DEventType::ExperimentStarted => {
            DReport::system_msg(event.at, "Experiment started")
        }

        DEventType::ExperimentCompleted => {
            DReport::system_msg(event.at, "Experiment completed")
        }

        DEventType::JobStarted { id } => {
            if let Some(job) = jobs.get(&id) {
                DReport::system_msg(event.at, format!("Job `{}` started", job.name))
            } else {
                warn!("Runner reported that it has started working on job #{}, which does not exist; this is probably a bug", id);
                return None;
            }
        }

        DEventType::JobCompleted { id, result } => {
            let result = if let Err(err) = result {
                format!("failure: {}", err)
            } else {
                "success".to_string()
            };

            if let Some(job) = jobs.get(&id) {
                DReport::system_msg(event.at, format!("Job `{}` completed with {}", job.name, result))
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