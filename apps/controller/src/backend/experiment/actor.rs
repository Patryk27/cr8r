use std::sync::Arc;

use chrono::{DateTime, Utc};
use log::*;
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

use lib_interop::domain::{DExperimentId, DJob, DReport};

use crate::backend::experiment::{ExperimentRx, ExperimentStatus};

pub struct ExperimentActor {
    pub id: DExperimentId,
    pub jobs: Vec<DJob>,
    pub created_at: DateTime<Utc>,
    pub watchers: Vec<mpsc::UnboundedSender<Arc<DReport>>>,
    pub status: ExperimentStatus,
}

impl ExperimentActor {
    pub async fn start(mut self, mut rx: ExperimentRx) {
        debug!("Actor started");
        debug!("-> id: {}", self.id);

        while let Some(msg) = rx.next().await {
            self.triage();
            msg.handle(&mut self);
        }

        debug!("Actor orphaned, halting");
    }

    fn triage(&mut self) {
        if let ExperimentStatus::Running { last_heartbeat_at, .. } = self.status {
            if (Utc::now() - last_heartbeat_at).num_minutes() >= 5 {
                self.status = ExperimentStatus::Zombie {
                    since: Utc::now(),
                };
            }
        }
    }
}