use chrono::{DateTime, Utc};

use lib_protocol::core::{PReport, PRunnerId};

#[derive(PartialEq)]
pub enum ExperimentStatus {
    AwaitingRunner {
        since: DateTime<Utc>,
    },

    Completed {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat: DateTime<Utc>,
        runner: PRunnerId,
        reports: Vec<PReport>,
        completed_scenarios: u32,
        total_scenarios: u32,
    },

    Zombie {
        since: DateTime<Utc>,
    },
}

impl Default for ExperimentStatus {
    fn default() -> Self {
        ExperimentStatus::AwaitingRunner {
            since: Utc::now(),
        }
    }
}