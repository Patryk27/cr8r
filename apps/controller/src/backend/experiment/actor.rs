use std::sync::Arc;

use chrono::{DateTime, Utc};
use log::*;
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

use lib_interop::domain::{DExperimentId, DJob, DReport};

use crate::backend::experiment::{ExperimentRx, ExperimentStatus};

pub struct ExperimentActor {
    rx: ExperimentRx,
    pub(super) id: DExperimentId,
    pub(super) jobs: Vec<DJob>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) watchers: Vec<mpsc::UnboundedSender<Arc<DReport>>>,
    pub(super) status: ExperimentStatus,
}

impl ExperimentActor {
    pub fn new(rx: ExperimentRx, id: DExperimentId, jobs: Vec<DJob>) -> Self {
        Self {
            rx,
            id,
            jobs,
            created_at: Utc::now(),
            watchers: Vec::new(),
            status: ExperimentStatus::default(),
        }
    }

    pub async fn start(mut self) {
        debug!("Actor started");
        debug!("-> id: {}", self.id);

        while let Some(msg) = self.rx.next().await {
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