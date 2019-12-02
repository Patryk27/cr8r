use futures_channel::{mpsc, oneshot};

use lib_protocol::core::{Assignment, ExperimentId, RunnerId, RunnerName, RunnerSecret};
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::backend::{Experiment, Result, Runner};

pub type SystemCommandTx = mpsc::UnboundedSender<SystemCommand>;
pub type SystemCommandRx = mpsc::UnboundedReceiver<SystemCommand>;

#[derive(Debug)]
pub enum SystemCommand {
    // ---------------------------- //
    // Assignment-oriented commands //

    RequestAssignment {
        runner: RunnerId,
        tx: oneshot::Sender<Result<Option<Assignment>>>,
    },

    // ---------------------------- //
    // Experiment-oriented commands //

    FindExperiment {
        experiment: ExperimentId,
        tx: oneshot::Sender<Result<Experiment>>,
    },

    FindExperiments {
        tx: oneshot::Sender<Vec<Experiment>>,
    },

    LaunchExperiment {
        experiment: ExperimentDefinitionInner,
        tx: oneshot::Sender<Result<ExperimentId>>,
    },

    // ------------------------ //
    // Runner-oriented commands //

    FindRunners {
        tx: oneshot::Sender<Vec<Runner>>,
    },

    RegisterRunner {
        name: RunnerName,
        secret: RunnerSecret,
        tx: oneshot::Sender<Result<RunnerId>>,
    },
}