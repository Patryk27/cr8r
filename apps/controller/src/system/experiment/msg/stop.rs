use chrono::Utc;

use super::super::{ExperimentActor, ExperimentStatus};

pub fn stop(actor: &mut ExperimentActor) {
    if let ExperimentStatus::Stopped { .. } = &actor.status {
        return;
    }

    if let ExperimentStatus::Running { .. } = &actor.status {
        // @todo kill watchers
        // @todo notify runner
    }

    actor.status = ExperimentStatus::Stopped {
        since: Utc::now(),
    };
}