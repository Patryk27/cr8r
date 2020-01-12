use std::collections::VecDeque;

use log::*;
use tokio::stream::StreamExt;

use lib_interop::domain::{DEvent, DExperimentId};

use crate::session::Session;

use super::ExperimentLoggerRx;

pub struct ExperimentLoggerActor {
    pub rx: ExperimentLoggerRx,
    pub session: Session,
    pub experiment_id: DExperimentId,
    pub pending_events: VecDeque<DEvent>,
}

impl ExperimentLoggerActor {
    pub async fn start(mut self) {
        debug!("Actor started");

        while let Some(msg) = self.rx.next().await {
            // @todo we should wake once in a while to process pending events

            msg.handle(&mut self)
                .await;
        }

        // @todo even if we're orphaned we should process pending events

        debug!("Actor orphaned, halting");
    }
}