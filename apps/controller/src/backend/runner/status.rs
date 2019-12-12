use chrono::{DateTime, Utc};

use lib_protocol::core::PExperimentId;

#[derive(PartialEq)]
pub enum RunnerStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Working {
        since: DateTime<Utc>,
        experiment: PExperimentId,
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