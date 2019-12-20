use std::sync::Arc;

use chrono::{DateTime, Utc};
use log::*;
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

use lib_protocol::core::{PExperimentId, PExperimentReport, PExperimentStep};

use crate::backend::experiment::{ExperimentRx, ExperimentStatus};

pub struct ExperimentActor {
    rx: ExperimentRx,
    pub(super) id: PExperimentId,
    pub(super) system: String,
    pub(super) toolchain: String,
    pub(super) steps: Vec<PExperimentStep>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) watchers: Vec<mpsc::UnboundedSender<Arc<PExperimentReport>>>,
    pub(super) status: ExperimentStatus,
}

impl ExperimentActor {
    pub fn new(
        rx: ExperimentRx,
        id: PExperimentId,
        system: String,
        toolchain: String,
        steps: Vec<PExperimentStep>,
    ) -> Self {
        Self {
            rx,
            id,
            system,
            toolchain,
            steps,
            created_at: Utc::now(),
            watchers: Vec::new(),
            status: ExperimentStatus::default(),
        }
    }

    pub async fn main(mut self) {
        debug!("Actor started");
        debug!("-> id: {}", self.id);
        debug!("-> system: {}", self.system);
        debug!("-> toolchain: {}", self.toolchain);
        debug!("-> steps: {}", self.steps.len());

        while let Some(msg) = self.rx.next().await {
            self.perform_triage();
            msg.process(&mut self);
        }

        debug!("Actor orphaned, halting");
    }

    fn perform_triage(&mut self) {
        if let ExperimentStatus::Running { last_heartbeat_at, .. } = self.status {
            if (Utc::now() - last_heartbeat_at).num_minutes() >= 5 {
                self.status = ExperimentStatus::Zombie {
                    since: Utc::now(),
                };
            }
        }
    }
}