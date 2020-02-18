use std::collections::{HashMap, VecDeque};

use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::models::DExperimentId;

use crate::system::{AttachmentStore, Compiler, Experiment};

use super::ExperimentStoreMsg;

pub struct ExperimentStoreActor {
    pub attachment_store: AttachmentStore,
    pub compiler: Compiler,
    pub experiments: HashMap<DExperimentId, Experiment>,
    pub waiting_experiments: VecDeque<DExperimentId>,
    pub next_id: DExperimentId,
}

impl ExperimentStoreActor {
    pub async fn start(mut self, mut mailbox: URx<ExperimentStoreMsg>) {
        trace!("Actor started");

        while let Some(msg) = mailbox.next().await {
            msg.handle(&mut self)
                .await;
        }

        trace!("Actor halted");
    }
}