use chrono::{DateTime, Utc};
use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::models::{DRunnerId, DRunnerName};

use super::{RunnerMsg, RunnerStatus};

pub struct RunnerActor {
    pub id: DRunnerId,
    pub name: DRunnerName,
    pub joined_at: DateTime<Utc>,
    pub last_heartbeat_at: DateTime<Utc>,
    pub status: RunnerStatus,
}

impl RunnerActor {
    pub async fn start(mut self, mut mailbox: URx<RunnerMsg>) {
        trace!("Actor started");
        trace!("-> id = {}", self.id);
        trace!("-> name = {}", self.name);

        while let Some(msg) = mailbox.next().await {
            if msg.handle(&mut self).actor_should_stop() {
                break;
            }
        }

        trace!("Actor is halting");

        if let RunnerStatus::Working { experiment, .. } = &self.status {
            experiment.stop();
        }

        trace!("Actor halted");
    }
}