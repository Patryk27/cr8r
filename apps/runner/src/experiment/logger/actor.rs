use std::collections::VecDeque;

use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::domain::{DEvent, DExperimentId};

use crate::session::Session;

use super::ExperimentLoggerMsg;

pub struct ExperimentLoggerActor {
    pub session: Session,
    pub experiment_id: DExperimentId,
    pub pending_events: VecDeque<DEvent>,
}

impl ExperimentLoggerActor {
    pub async fn start(mut self, mut mailbox: URx<ExperimentLoggerMsg>) {
        debug!("Actor has started");

        while let Some(msg) = mailbox.next().await {
            // @todo we should wake once in a while to process events from the `pending_events` queue

            msg.handle(&mut self)
                .await;
        }

        // @todo even if we're orphaned, we should process events from the `pending_events` queue, not just drop them

        debug!("Actor has halted");
    }
}