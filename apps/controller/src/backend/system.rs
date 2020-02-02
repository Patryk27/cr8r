use anyhow::*;
use tokio::sync::mpsc;

use lib_core_actor::*;
use lib_interop::domain::{DAssignment, DDefinition, DExperimentId, DRunnerId, DRunnerName};

use crate::backend::{Compiler, Experiment, Runner};

use self::{
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

        tokio::spawn(SystemActor {
            compiler,
            runners: Runners::new(system.clone()),
            experiments: Experiments::new(),
        }.start(rx));

        system
    }
}

/// Runner-oriented impls
impl System {
    pub async fn create_runner(&self, name: DRunnerName) -> Result<DRunnerId> {
        ask!(self.tx, SystemMsg::CreateRunner { name })
    }

    pub async fn find_runners(&self) -> Vec<Runner> {
        ask!(self.tx, SystemMsg::FindRunners)
    }
}

/// Experiment-oriented impls
impl System {
    pub async fn create_experiment(&self, definition: DDefinition) -> Result<DExperimentId> {
        ask!(self.tx, SystemMsg::CreateExperiment { definition })
    }

    pub async fn find_experiment(&self, id: DExperimentId) -> Result<Experiment> {
        ask!(self.tx, SystemMsg::FindExperiment { id })
    }

    pub async fn find_experiments(&self) -> Vec<Experiment> {
        ask!(self.tx, SystemMsg::FindExperiments)
    }
}

/// Assignment-oriented impls
impl System {
    pub async fn get_assignment(&self, runner_id: DRunnerId) -> Result<Option<DAssignment>> {
        ask!(self.tx, SystemMsg::GetAssignment { runner_id })
    }
}
