use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{PRunnerId, PRunnerName};

use crate::backend::runner::{RunnerRx, RunnerStatus};
use crate::backend::System;

pub struct RunnerActor {
    rx: RunnerRx,
    pub(super) system: System,
    pub(super) id: PRunnerId,
    pub(super) name: PRunnerName,
    pub(super) joined_at: DateTime<Utc>,
    pub(super) heartbeaten_at: DateTime<Utc>,
    pub(super) status: RunnerStatus,
}

impl RunnerActor {
    pub fn new(rx: RunnerRx, system: System, id: PRunnerId, name: PRunnerName) -> Self {
        Self {
            rx,
            system,
            id,
            name,
            joined_at: Utc::now(),
            heartbeaten_at: Utc::now(),
            status: RunnerStatus::default(),
        }
    }

    pub async fn main(mut self) {
        debug!("Actor started");
        debug!("-> id: {}", self.id);
        debug!("-> name: {}", self.name);

        while let Some(msg) = self.rx.next().await {
            msg.process(&mut self);
        }

        debug!("Actor orphaned, halting");
    }
}