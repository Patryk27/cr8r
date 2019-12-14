use std::sync::Arc;

use chrono::{DateTime, Utc};

use lib_protocol::core::{PReport, PRunnerId};

#[derive(PartialEq)]
pub enum ExperimentStatus {
    AwaitingRunner {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat_at: DateTime<Utc>,
        runner: PRunnerId,
        reports: Vec<Arc<PReport>>,
        completed_scenarios: u32,
    },

    Completed {
        since: DateTime<Utc>,
        success: bool,
    },

    Aborted {
        since: DateTime<Utc>,
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