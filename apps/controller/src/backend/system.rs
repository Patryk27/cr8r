use tokio::sync::mpsc;

use lib_actor::ask;
use lib_interop::contract::{CAssignment, CExperimentDefinition, CExperimentId, CRunnerId, CRunnerName};

use crate::backend::{Compiler, Experiment, Result, Runner};

pub(self) use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone, Debug)]
pub struct System {
    tx: SystemTx,
}

impl System {
    pub fn new(compiler: Compiler) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let system = Self { tx };

        tokio::spawn(SystemActor::new(
            rx,
            system.clone(),
            compiler,
        ).main());

        system
    }
}

/// Runner-oriented impls
impl System {
    pub async fn create_runner(&self, name: CRunnerName) -> Result<CRunnerId> {
        ask!(self.tx, SystemMsg::CreateRunner { name })
    }

    pub async fn find_runners(&self) -> Vec<Runner> {
        ask!(self.tx, SystemMsg::FindRunners)
    }
}

/// Experiment-oriented impls
impl System {
    pub async fn create_experiment(&self, def: CExperimentDefinition) -> Result<CExperimentId> {
        ask!(self.tx, SystemMsg::CreateExperiment { def })
    }

    pub async fn find_experiment(&self, id: CExperimentId) -> Result<Experiment> {
        ask!(self.tx, SystemMsg::FindExperiment { id })
    }

    pub async fn find_experiments(&self) -> Vec<Experiment> {
        ask!(self.tx, SystemMsg::FindExperiments)
    }
}

/// Assignment-oriented impls
impl System {
    pub async fn get_assignment(&self, runner_id: CRunnerId) -> Result<Option<CAssignment>> {
        ask!(self.tx, SystemMsg::GetAssignment { runner_id })
    }
}
