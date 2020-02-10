use std::collections::{HashMap, VecDeque};

use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::domain::DExperimentId;

use crate::system::{Compiler, Experiment};

use super::ExperimentsMsg;

pub struct ExperimentsActor {
    pub compiler: Compiler,
    pub experiments: HashMap<DExperimentId, Experiment>,
    pub pending_ids: VecDeque<DExperimentId>,
    pub next_id: DExperimentId,
}

impl ExperimentsActor {
    pub async fn start(mut self, mut mailbox: URx<ExperimentsMsg>) {
        trace!("Actor started");

        while let Some(msg) = mailbox.next().await {
            msg.handle(&mut self);
        }

        trace!("Actor halted");
    }
}