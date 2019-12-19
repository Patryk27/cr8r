use std::sync::Arc;

use chrono::{DateTime, Utc};

use lib_protocol::core::{PExperimentEvent, PExperimentReport, PRunnerId};

#[derive(PartialEq)]
pub enum ExperimentStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat_at: DateTime<Utc>,
        runner: PRunnerId,
        events: Vec<Arc<PExperimentEvent>>,
        reports: Vec<Arc<PExperimentReport>>,
        completed_steps: u32,
    },

    Completed {
        since: DateTime<Utc>,
        reports: Vec<Arc<PExperimentReport>>,
        result: Result<(), String>,
    },

    Zombie {
        since: DateTime<Utc>,
    },
}

impl Default for ExperimentStatus {
    fn default() -> Self {
        ExperimentStatus::Idle {
            since: Utc::now(),
        }
    }
}