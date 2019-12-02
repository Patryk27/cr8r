use futures_channel::mpsc;

use lib_protocol::core::{Assignment, ExperimentId, RunnerId, RunnerName, RunnerSecret};
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::backend::{Compiler, Experiment, Result, Runner};
use crate::msg;

pub use self::{
    actor::*,
    command::*,
};

mod actor;
mod command;

#[derive(Clone, Debug)]
pub struct System {
    tx: SystemCommandTx,
}

impl System {
    pub fn spawn(runner_secret: RunnerSecret, compiler: Compiler) -> Self {
        let (tx, rx) = mpsc::unbounded();
        let system = Self { tx };

        tokio::spawn(SystemActor::new(
            system.clone(),
            runner_secret,
            compiler,
        ).start(rx));

        system
    }
}

/// Assignment-oriented impls
impl System {
    pub async fn request_assignment(&self, runner: RunnerId) -> Result<Option<Assignment>> {
        msg!(self.tx, tx, SystemCommand::RequestAssignment { runner, tx })
    }
}

/// Experiment-oriented impls
impl System {
    pub async fn find_experiment(&self, experiment: ExperimentId) -> Result<Experiment> {
        msg!(self.tx, tx, SystemCommand::FindExperiment { experiment, tx })
    }

    pub async fn find_experiments(&self) -> Vec<Experiment> {
        msg!(self.tx, tx, SystemCommand::FindExperiments { tx })
    }

    pub async fn launch_experiment(&self, experiment: ExperimentDefinitionInner) -> Result<ExperimentId> {
        msg!(self.tx, tx, SystemCommand::LaunchExperiment { experiment, tx })
    }
}

/// Runner-oriented impls
impl System {
    pub async fn find_runners(&self) -> Vec<Runner> {
        msg!(self.tx, tx, SystemCommand::FindRunners { tx })
    }

    pub async fn register_runner(&self, name: RunnerName, secret: RunnerSecret) -> Result<RunnerId> {
        msg!(self.tx, tx, SystemCommand::RegisterRunner { name, secret, tx })
    }
}