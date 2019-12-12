use lib_protocol::core::{PReport, PRunnerId};

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};
use crate::backend::Result;

pub fn process(actor: &mut ExperimentActor, runner: PRunnerId, report: PReport) -> Result<()> {
    match &mut actor.status {
        ExperimentStatus::AwaitingRunner { .. } => {
            Err("This experiment has not yet been started".into())
        }

        ExperimentStatus::Running {
            runner: expected_runner,
            reports,
            ..
        } => {
            if &runner != expected_runner {
                return Err("Specified runner is not allowed to report on this experiment".into());
            }

            if let Some(watcher) = &mut actor.watcher {
                watcher.add(report.clone());
            }

            reports.push(report);

            // @todo increase total / completed

            Ok(())
        }

        ExperimentStatus::Completed { .. } => {
            Err("This experiment has been already completed".into())
        }

        ExperimentStatus::Aborted { .. } => {
            Err("This experiment has been aborted".into())
        }

        ExperimentStatus::Zombie { .. } => {
            Err("This experiment has been abandoned by its runner and has become a zombie - it can be only aborted or restarted".into())
        }
    }
}