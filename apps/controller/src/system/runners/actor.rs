use std::collections::HashMap;

use bimap::BiMap;
use log::*;
use tokio::stream::StreamExt;

use lib_core_channel::URx;
use lib_interop::domain::{DRunnerId, DRunnerName};

use crate::system::Runner;

use super::RunnersMsg;

#[derive(Default)]
pub struct RunnersActor {
    pub index: BiMap<DRunnerId, DRunnerName>,
    pub runners: HashMap<DRunnerId, Runner>,
    pub next_id: DRunnerId,
}

impl RunnersActor {
    pub async fn start(mut self, mut mailbox: URx<RunnersMsg>) {
        trace!("Actor started");

        while let Some(msg) = mailbox.next().await {
            msg.handle(&mut self);
        }

        trace!("Actor halted");
    }
}