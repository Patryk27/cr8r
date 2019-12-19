use chrono::Utc;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};

pub fn abort(actor: &mut ExperimentActor) {
    if let ExperimentStatus::Running { reports, .. } = &actor.status {
        // @todo kill watchers

        actor.status = ExperimentStatus::Completed {
            since: Utc::now(),
            reports: reports.to_vec(),
            result: Err("Experiment has been aborted".into()),
        };
    }
}