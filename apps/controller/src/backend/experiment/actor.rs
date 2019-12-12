use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{PExperimentId, PScenario};

use crate::backend::{ExperimentWatcher, System};
use crate::backend::experiment::{ExperimentRx, ExperimentStatus};

pub struct ExperimentActor {
    rx: ExperimentRx,
    pub(super) system: System,
    pub(super) experiment: PExperimentId,
    pub(super) scenarios: Vec<PScenario>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) watcher: Option<ExperimentWatcher>,
    pub(super) status: ExperimentStatus,
}

impl ExperimentActor {
    pub fn new(
        rx: ExperimentRx,
        system: System,
        experiment: PExperimentId,
        scenarios: Vec<PScenario>,
    ) -> Self {
        Self {
            rx,
            system,
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
            self.maybe_zombify();
            msg.process(&mut self);
        }

        debug!("Actor orphaned, halting");
    }

    fn maybe_zombify(&mut self) {
        let zombify = match self.status {
            ExperimentStatus::Running { last_heartbeat, .. } => {
                (Utc::now() - last_heartbeat).num_minutes() >= 5
            }

            _ => false,
        };

        if zombify {
            self.status = ExperimentStatus::Zombie {
                since: Utc::now(),
            }
        }
    }
}