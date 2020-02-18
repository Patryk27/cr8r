use std::sync::Arc;

use chrono::{DateTime, Utc};

use lib_interop::models::{DEvent, DReport, DRunnerId};

#[derive(PartialEq)]
pub enum ExperimentStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat_at: DateTime<Utc>,
        runner: DRunnerId,
        events: Vec<Arc<DEvent>>,
        reports: Vec<Arc<DReport>>,
        completed_jobs: u32,
    },

    Completed {
        since: DateTime<Utc>,
        reports: Vec<Arc<DReport>>,
        result: Result<(), String>,
    },

    Stopped {
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