use chrono::{DateTime, Utc};

use lib_interop::models::DExperimentId;

use crate::system::Experiment;

pub enum RunnerStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Working {
        since: DateTime<Utc>,
        experiment: Experiment,
        experiment_id: DExperimentId,
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