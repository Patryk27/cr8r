use chrono::{DateTime, Utc};
use log::*;
use tokio::stream::StreamExt;

use lib_actor::ActorWorkflow;
use lib_interop::domain::{DRunnerId, DRunnerName};

use crate::backend::runner::{RunnerRx, RunnerStatus};
use crate::backend::System;

pub struct RunnerActor {
    rx: RunnerRx,
    pub(super) system: System,
    pub(super) id: DRunnerId,
    pub(super) name: DRunnerName,
    pub(super) joined_at: DateTime<Utc>,
    pub(super) last_heartbeat_at: DateTime<Utc>,
    pub(super) status: RunnerStatus,
}

impl RunnerActor {
    pub fn new(rx: RunnerRx, system: System, id: DRunnerId, name: DRunnerName) -> Self {
        Self {
            rx,
            system,
            id,
            name,
            joined_at: Utc::now(),
            last_heartbeat_at: Utc::now(),
            status: RunnerStatus::default(),
        }
    }

    pub async fn start(mut self) {
        debug!("Actor started");
        debug!("-> id: {}", self.id);
        debug!("-> name: {}", self.name);

        while let Some(msg) = self.rx.next().await {
            match msg.handle(&mut self) {
                ActorWorkflow::Continue => {
                    //
                }

                ActorWorkflow::Stop => {
                    debug!("Actor killed");
                    return self.on_killed();
                }
            }
        }

        debug!("Actor orphaned, halting");
    }

    fn on_killed(self) {
        if let RunnerStatus::Working { experiment, .. } = self.status {
            experiment.abort();
        }
    }
}