use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{PExperimentId, PScenario};

use crate::backend::experiment::{ExperimentRx, ExperimentStatus};
use crate::backend::ExperimentWatcher;

pub struct ExperimentActor {
    rx: ExperimentRx,
    pub(super) experiment: PExperimentId,
    pub(super) scenarios: Vec<PScenario>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) watcher: Option<ExperimentWatcher>,
    pub(super) status: ExperimentStatus,
}

impl ExperimentActor {
    pub fn new(
        rx: ExperimentRx,
        experiment: PExperimentId,
        scenarios: Vec<PScenario>,
    ) -> Self {
        Self {
            rx,
            experiment,
            scenarios,
            created_at: Utc::now(),
            watcher: None,
            status: ExperimentStatus::default(),
        }
    }

    pub async fn main(mut self) {
        debug!("Actor started");
        debug!("-> experiment: {}", self.experiment);
        debug!("-> scenarios: {}", self.scenarios.len());

        while let Some(msg) = self.rx.next().await {
            self.triage();
            msg.process(&mut self);
        }

        debug!("Actor orphaned, halting");
    }

    fn triage(&mut self) {
        match self.status {
            ExperimentStatus::Running { last_heartbeat_at, .. } => {
                if (Utc::now() - last_heartbeat_at).num_minutes() >= 5 {
                    self.status = ExperimentStatus::Zombie {
                        since: Utc::now(),
                    };
                }
            }

            _ => (),
        };
    }
}