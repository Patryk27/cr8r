use std::sync::Arc;

use chrono::{DateTime, Utc};
use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::{URx, UTx};
use lib_interop::domain::{DExperimentId, DJob, DReport};

use super::{ExperimentMsg, ExperimentStatus};

pub struct ExperimentActor {
    pub id: DExperimentId,
    pub jobs: Vec<DJob>,
    pub created_at: DateTime<Utc>,
    pub watchers: Vec<UTx<Arc<DReport>>>,
    pub status: ExperimentStatus,
}

impl ExperimentActor {
    pub async fn start(mut self, mut mailbox: URx<ExperimentMsg>) {
        trace!("Actor started");
        trace!("-> id = {}", self.id);

        while let Some(msg) = mailbox.next().await {
            self.perform_self_check();

            msg.handle(&mut self);
        }

        trace!("Actor halted");
    }

    fn perform_self_check(&mut self) {
        if let ExperimentStatus::Running { last_heartbeat_at, .. } = self.status {
            if (Utc::now() - last_heartbeat_at).num_minutes() >= 10 {
                warn!("Experiment [id={}] has been running for over 10 minutes without any information from the runner - stopping it", self.id);

                ExperimentMsg::Stop
                    .handle(self);
            }
        }
    }
}