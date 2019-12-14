use std::sync::Arc;

use chrono::Utc;

use lib_protocol::core::{PReport, PRunnerId};
use lib_protocol::core::p_report::Op;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn process(actor: &mut ExperimentActor, runner: PRunnerId, report: PReport) -> Result<()> {
    match &mut actor.status {
        ExperimentStatus::AwaitingRunner { .. } => {
            Err("This experiment has not yet been started".into())
        }

        ExperimentStatus::Running {
            runner: experiment_runner,
            reports,
            completed_scenarios,
            ..
        } => {
            if &runner != experiment_runner {
                return Err("Given runner is not allowed to report on this experiment".into());
            }

            let report = Arc::new(report);

            if let Some(watcher) = &mut actor.watcher {
                watcher.push_report(Arc::clone(&report));
            }

            reports.push(Arc::clone(&report));

            // A few reports convey special meaning - for instance when we receive "experiment completed", we have to
            // adjust this actor' state, so that it reflects that, well, the experiment's completed
            if let Some(op) = &report.op {
                match op {
                    Op::ExperimentCompleted(_) => {
                        let success = reports
                            .iter()
                            .filter_map(|report| match report.op.as_ref()? {
                                Op::ScenarioCompleted(op) => Some(op),
                                _ => None,
                            })
                            .all(|op| op.success);

                        actor.status = ExperimentStatus::Completed {
                            since: Utc::now(),
                            success,
                        };

                        // Since the experiment's done, no more data will be fed to the watcher and so there's no point
                        // in keeping it alive
                        if let Some(mut watcher) = actor.watcher.take() {
                            watcher.kill();
                        }
                    }

                    Op::ExperimentAborted(_) => {
                        actor.status = ExperimentStatus::Aborted {
                            since: Utc::now(),
                        };

                        // Since the experiment's done, no more data will be fed to the watcher and so there's no point
                        // in keeping it alive
                        if let Some(mut watcher) = actor.watcher.take() {
                            watcher.kill();
                        }
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
