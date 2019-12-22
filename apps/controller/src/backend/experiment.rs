use std::sync::Arc;

use tokio::sync::mpsc;

use lib_actor::{ask, tell};
use lib_interop::contract::{CAssignment, CEvent, CExperiment, CExperimentId, CProgram, CReport, CRunnerId};

use crate::backend::Result;

pub(self) use self::{
    actor::*,
    msg::*,
    status::*,
};

mod actor;
mod msg;
mod status;

#[derive(Clone, Debug)]
pub struct Experiment {
    tx: ExperimentTx,
}

impl Experiment {
    pub fn new(id: CExperimentId, program: CProgram) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(ExperimentActor::new(
            rx,
            id,
            program,
        ).main());

        Self { tx }
    }

    // @todo there should be something like `CExperimentAbortReason`
    pub fn abort(&self) {
        tell!(self.tx, ExperimentMsg::Abort);
    }

    pub async fn add_event(&self, runner_id: CRunnerId, event: CEvent) -> Result<()> {
        ask!(self.tx, ExperimentMsg::AddEvent { runner_id, event })
    }

    pub async fn get_model(&self) -> CExperiment {
        ask!(self.tx, ExperimentMsg::GetModel)
    }

    pub async fn get_reports(&self) -> Vec<Arc<CReport>> {
        ask!(self.tx, ExperimentMsg::GetReports)
    }

    pub async fn start(&self, runner_id: CRunnerId) -> Result<CAssignment> {
        ask!(self.tx, ExperimentMsg::Start { runner_id })
    }

    pub async fn watch(&self) -> Result<mpsc::UnboundedReceiver<Arc<CReport>>> {
        ask!(self.tx, ExperimentMsg::Watch)
    }
}