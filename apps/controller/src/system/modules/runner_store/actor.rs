use std::collections::HashMap;

use bimap::BiMap;
use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::models::{DRunnerId, DRunnerName};

use crate::system::{Runner, SystemEventBus};

use super::RunnerStoreMsg;

pub struct RunnerStoreActor {
    pub bus: SystemEventBus,
    pub index: BiMap<DRunnerId, DRunnerName>,
    pub runners: HashMap<DRunnerId, Runner>,
    pub next_id: DRunnerId,
}

impl RunnerStoreActor {
    pub async fn start(mut self, mut mailbox: URx<RunnerStoreMsg>) {
        trace!("Actor started");

        while let Some(msg) = mailbox.next().await {
            msg.handle(&mut self);
        }

        trace!("Actor halted");
    }
}