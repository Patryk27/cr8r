use std::collections::VecDeque;

use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::models::{DEvent, DExperimentId};

use crate::rpc::Session;

use super::LoggerMsg;

pub struct LoggerActor {
    pub session: Session,
    pub experiment_id: DExperimentId,
    pub pending_events: VecDeque<DEvent>,
}

impl LoggerActor {
    pub async fn start(mut self, mut mailbox: URx<LoggerMsg>) {
        trace!("Actor started");

        while let Some(msg) = mailbox.next().await {
            // @todo we should wake once in a while to process events from the `pending_events` queue

            msg.handle(&mut self)
                .await;
        }

        // @todo even if we're orphaned, we should process events from the `pending_events` queue, not just drop them

        trace!("Actor halted");
    }
}