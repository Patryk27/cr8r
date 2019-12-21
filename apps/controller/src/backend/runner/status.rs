use chrono::{DateTime, Utc};

use lib_interop::protocol::core::PExperimentId;

use crate::backend::Experiment;

pub enum RunnerStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Working {
        since: DateTime<Utc>,
        experiment: Experiment,
        experiment_id: PExperimentId,
    },

    Zombie {
        since: DateTime<Utc>,
    },
}

impl Default for RunnerStatus {
    fn default() -> Self {
        RunnerStatus::Idle {
            since: Utc::now(),
        }
    }
}