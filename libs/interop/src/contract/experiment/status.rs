use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub enum CExperimentStatus {
    Idle {
        since: DateTime<Utc>,
    },

    Running {
        since: DateTime<Utc>,
        last_heartbeat_at: DateTime<Utc>,
        completed_steps: u32,
    },

    Completed {
        since: DateTime<Utc>,
        result: Result<(), String>,
    },

    Zombie {
        since: DateTime<Utc>,
    },
}