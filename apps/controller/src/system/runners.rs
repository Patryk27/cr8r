use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::domain::{DRunnerId, DRunnerName};

use crate::system::Runner;

use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct Runners {
    tx: UTx<RunnersMsg>,
}

impl Runners {
    pub fn new() -> Self {
        let (tx, rx) = unbounded_channel();

        spawn(
            RunnersActor::default()
                .start(rx)
        );

        Self { tx }
    }

    pub async fn find_all(&self) -> Vec<Runner> {
        ask!(self.tx, RunnersMsg::FindAll)
    }

    pub async fn register(&self, name: DRunnerName) -> Result<DRunnerId> {
        ask!(self.tx, RunnersMsg::Register { name })
    }
}
