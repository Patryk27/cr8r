use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{RunnerId, RunnerName};

use crate::backend::{RunnerCommandRx, System};

pub struct RunnerActor {
    system: System,
    id: RunnerId,
    name: RunnerName,
    heartbeat: DateTime<Utc>,
}

impl RunnerActor {
    pub fn new(system: System, id: RunnerId, name: RunnerName) -> Self {
        Self {
            system,
            id,
            name,
            heartbeat: Utc::now(),
        }
    }

    pub async fn start(mut self, mut rx: RunnerCommandRx) {
        debug!("Runner actor started, entering the event loop");
        debug!("-> id: {}", self.id);
        debug!("-> name: {}", self.name);

        while let Some(cmd) = rx.next().await {
            debug!("Processing command: {:?}", cmd);
        }

        debug!("Runner actor has been orphaned, halting it");
    }
}