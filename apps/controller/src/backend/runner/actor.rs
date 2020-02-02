use chrono::{DateTime, Utc};
use log::*;
use tokio::stream::StreamExt;

use lib_core_actor::*;
use lib_interop::domain::{DRunnerId, DRunnerName};

use crate::backend::runner::{RunnerRx, RunnerStatus};
use crate::backend::System;

pub struct RunnerActor {
    pub system: System,
    pub id: DRunnerId,
    pub name: DRunnerName,
    pub joined_at: DateTime<Utc>,
    pub last_heartbeat_at: DateTime<Utc>,
    pub status: RunnerStatus,
}

impl RunnerActor {
    pub async fn start(mut self, mut rx: RunnerRx) {
        debug!("Actor started");
        debug!("-> id: {}", self.id);
        debug!("-> name: {}", self.name);

        while let Some(msg) = rx.next().await {
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