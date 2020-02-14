use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::domain::{DDefinition, DExperimentId};

use crate::system::{Compiler, Experiment};

use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct Experiments {
    tx: UTx<ExperimentsMsg>,
}

impl Experiments {
    pub fn new(compiler: Compiler) -> Self {
        let (tx, rx) = unbounded_channel();

        spawn(ExperimentsActor {
            compiler,
            experiments: Default::default(),
            pending_ids: Default::default(),
            next_id: Default::default(),
        }.start(rx));

        Self { tx }
    }

    pub async fn delete(&self, id: DExperimentId) -> Result<()> {
        ask!(self.tx, ExperimentsMsg::Delete { id })
    }

    pub async fn find_all(&self) -> Vec<Experiment> {
        ask!(self.tx, ExperimentsMsg::FindAll)
    }

    pub async fn find_one(&self, id: DExperimentId) -> Result<Experiment> {
        ask!(self.tx, ExperimentsMsg::FindOne { id })
    }

    pub async fn launch(&self, definition: DDefinition) -> DExperimentId {
        ask!(self.tx, ExperimentsMsg::Launch { definition })
    }
}