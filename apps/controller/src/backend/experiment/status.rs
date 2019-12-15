use std::sync::Arc;

use chrono::{DateTime, Utc};

use lib_protocol::core::{PExperimentEvent, PExperimentReport, PRunnerId};

#[derive(PartialEq)]
pub enum ExperimentStatus {
    AwaitingRunner {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat_at: DateTime<Utc>,
        runner: PRunnerId,
        events: Vec<Arc<PExperimentEvent>>,
        reports: Vec<Arc<PExperimentReport>>,
        completed_scenarios: u32,
    },

    Completed {
        since: DateTime<Utc>,
        reports: Vec<Arc<PExperimentReport>>,
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