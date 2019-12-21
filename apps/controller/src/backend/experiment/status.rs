use std::sync::Arc;

use chrono::{DateTime, Utc};

use lib_interop::contract::{CEvent, CReport, CRunnerId};

#[derive(PartialEq)]
pub enum ExperimentStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat_at: DateTime<Utc>,
        runner: CRunnerId,
        events: Vec<Arc<CEvent>>,
        reports: Vec<Arc<CReport>>,
        completed_steps: u32,
    },

    Completed {
        since: DateTime<Utc>,
        reports: Vec<Arc<CReport>>,
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