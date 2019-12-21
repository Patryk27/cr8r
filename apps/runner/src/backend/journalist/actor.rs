use std::collections::VecDeque;

use log::*;
use tokio::stream::StreamExt;

use lib_interop::contract::CEvent;

use crate::backend::journalist::JournalistRx;
use crate::core::ExperimentClient;

pub struct JournalistActor {
    rx: JournalistRx,
    pub(super) client: ExperimentClient,
    pub(super) pending_events: VecDeque<CEvent>,
}

impl JournalistActor {
    pub fn new(rx: JournalistRx, client: ExperimentClient) -> Self {
        Self {
            rx,
            client,
            pending_events: VecDeque::new(),
        }
    }

    pub async fn main(mut self) {
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