use std::sync::Arc;

use chrono::{DateTime, Utc};
use futures_channel::mpsc;
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{PExperimentId, PExperimentReport, PScenario};

use crate::backend::experiment::{ExperimentRx, ExperimentStatus};

pub struct ExperimentActor {
    rx: ExperimentRx,
    pub(super) experiment: PExperimentId,
    pub(super) scenarios: Vec<PScenario>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) watchers: Vec<mpsc::UnboundedSender<Arc<PExperimentReport>>>,
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
            watchers: Vec::new(),
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