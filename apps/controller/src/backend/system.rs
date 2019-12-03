use futures_channel::mpsc;

use lib_actor::ask;
use lib_protocol::core::{Assignment, ExperimentId, RunnerId, RunnerName};
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::backend::{Compiler, Experiment, Result, Runner};

pub use self::{
    actor::*,
    message::*,
};

mod actor;
mod message;

#[derive(Clone, Debug)]
pub struct System {
    tx: SystemTx,
}

impl System {
    pub fn spawn(compiler: Compiler) -> Self {
        let (tx, rx) = mpsc::unbounded();
        let system = Self { tx };

        tokio::spawn(SystemActor::new(
            rx,
            system.clone(),
            compiler,
        ).start());

        system
    }
}

/// Assignment-oriented impls
impl System {
    pub async fn request_assignment(&self, runner: RunnerId) -> Result<Option<Assignment>> {
        ask!(self.tx, SystemMsg::RequestAssignment { runner })
    }
}

/// Experiment-oriented impls
impl System {
    pub async fn find_experiment(&self, experiment: ExperimentId) -> Result<Experiment> {
        ask!(self.tx, SystemMsg::FindExperiment { experiment })
    }

    pub async fn find_experiments(&self) -> Vec<Experiment> {
        ask!(self.tx, SystemMsg::FindExperiments)
    }

    pub async fn launch_experiment(&self, experiment: ExperimentDefinitionInner) -> Result<ExperimentId> {
        ask!(self.tx, SystemMsg::LaunchExperiment { experiment })
    }
}

/// Runner-oriented impls
impl System {
    pub async fn find_runners(&self) -> Vec<Runner> {
        ask!(self.tx, SystemMsg::FindRunners)
    }

    pub async fn register_runner(&self, name: RunnerName) -> Result<RunnerId> {
        ask!(self.tx, SystemMsg::RegisterRunner { name })
    }
}