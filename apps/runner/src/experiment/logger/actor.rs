use std::collections::VecDeque;

use log::*;
use tokio::stream::StreamExt;

use lib_interop::domain::{DEvent, DExperimentId};

use crate::session::Session;

use super::ExperimentLoggerRx;

pub struct ExperimentLoggerActor {
    pub session: Session,
    pub experiment_id: DExperimentId,
    pub pending_events: VecDeque<DEvent>,
}

impl ExperimentLoggerActor {
    pub async fn start(mut self, mut rx: ExperimentLoggerRx) {
        debug!("Actor started");

        while let Some(msg) = rx.next().await {
            // @todo we should wake once in a while to process events from the `pending_events` queue

            msg.handle(&mut self)
                .await;
        }

        // @todo even if we're orphaned, we should process events from the `pending_events` queue, not just drop them

        debug!("Actor orphaned, halting");
    }
}