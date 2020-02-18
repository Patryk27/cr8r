use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::models::{DDefinition, DExperimentId, DRunnerId};

use crate::system::{AttachmentStore, Compiler, Experiment};

use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct ExperimentStore {
    tx: UTx<ExperimentStoreMsg>,
}

impl ExperimentStore {
    pub fn new(attachment_store: AttachmentStore, compiler: Compiler) -> Self {
        let (tx, rx) = unbounded_channel();

        spawn(ExperimentStoreActor {
            attachment_store,
            compiler,
            experiments: Default::default(),
            waiting_experiments: Default::default(),
            next_id: Default::default(),
        }.start(rx));

        Self { tx }
    }

    pub async fn delete(&self, id: DExperimentId) -> Result<()> {
        ask!(self.tx, ExperimentStoreMsg::Delete { id })
    }

    pub async fn find_all(&self) -> Vec<Experiment> {
        ask!(self.tx, ExperimentStoreMsg::FindAll)
    }

    pub async fn find_one(&self, id: DExperimentId) -> Result<Experiment> {
        ask!(self.tx, ExperimentStoreMsg::FindOne { id })
    }

    pub async fn launch(&self, definition: DDefinition) -> Result<DExperimentId> {
        ask!(self.tx, ExperimentStoreMsg::Launch { definition })
    }

    pub async fn prepare_assignment(&self, runner_id: DRunnerId) -> Result<Option<DExperimentId>> {
        ask!(self.tx, ExperimentStoreMsg::PrepareAssignment { runner_id })
    }
}