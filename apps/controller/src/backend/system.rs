use tokio::sync::mpsc;

use lib_actor::ask;
use lib_protocol::core::{PAssignment, PExperimentId, PRunnerId, PRunnerName};
use lib_protocol::core::p_experiment_def::Op as PExperimentDefOp;

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
    pub fn spawn(compiler: Compiler) -> Self {
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

/// Assignment-oriented impls
impl System {
    pub async fn request_assignment(&self, runner: PRunnerId) -> Result<Option<PAssignment>> {
        ask!(self.tx, SystemMsg::RequestAssignment { runner })
    }
}

/// Experiment-oriented impls
impl System {
    pub async fn find_experiment(&self, experiment: PExperimentId) -> Result<Experiment> {
        ask!(self.tx, SystemMsg::FindExperiment { experiment })
    }

    pub async fn find_experiments(&self) -> Vec<Experiment> {
        ask!(self.tx, SystemMsg::FindExperiments)
    }

    pub async fn launch_experiment(&self, experiment_def: PExperimentDefOp) -> Result<PExperimentId> {
        ask!(self.tx, SystemMsg::LaunchExperiment { experiment_def })
    }
}

/// Runner-oriented impls
impl System {
    pub async fn find_runners(&self) -> Vec<Runner> {
        ask!(self.tx, SystemMsg::FindRunners)
    }

    pub async fn register_runner(&self, name: PRunnerName) -> Result<PRunnerId> {
        ask!(self.tx, SystemMsg::RegisterRunner { name })
    }
}