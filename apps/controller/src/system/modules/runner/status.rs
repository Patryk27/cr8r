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
        previous_status: Box<RunnerStatus>,
    },
}

impl RunnerStatus {
    pub fn is_zombie(&self) -> bool {
        match &self {
            RunnerStatus::Zombie { .. } => true,
            _ => false,
        }
    }
}

impl Default for RunnerStatus {
    fn default() -> Self {
        RunnerStatus::Idle {
            since: Utc::now(),
        }
    }
}